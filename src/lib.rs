#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error as PestError;
use pest::error::ErrorVariant;
use pest::iterators::Pairs;
use pest::Parser;
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
pub struct PipelineParser;

pub fn parse_file(path: &PathBuf) -> Result<(), pest::error::Error<Rule>> {
    use std::fs::File;
    use std::io::Read;

    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();

            if let Err(e) = file.read_to_string(&mut contents) {
                return Err(PestError::new_from_pos(
                    ErrorVariant::CustomError {
                        message: format!("{}", e),
                    },
                    pest::Position::from_start(""),
                ));
            } else {
                return parse_pipeline_string(&contents);
            }
        }
        Err(e) => {
            return Err(PestError::new_from_pos(
                ErrorVariant::CustomError {
                    message: format!("{}", e),
                },
                pest::Position::from_start(""),
            ));
        }
    }
}

pub fn parse_graph_stages(path: &PathBuf) -> Result<(), PestError<Rule>> {
    use std::fs::File;
    use std::io::Read;

    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();

            if let Err(e) = file.read_to_string(&mut contents) {
                return Err(PestError::new_from_pos(
                    ErrorVariant::CustomError {
                        message: format!("{}", e),
                    },
                    pest::Position::from_start(""),
                ));
            } else {
                println!("{}", build_dot_file(stage_graphs(&contents)?));
                Ok(())
            }
        }
        Err(e) => {
            return Err(PestError::new_from_pos(
                ErrorVariant::CustomError {
                    message: format!("{}", e),
                },
                pest::Position::from_start(""),
            ));
        }
    }
}

fn build_dot_file<'a>(stages: Vec<&str>) -> String {
    let nodes = stages.windows(2).fold("".to_string(), |acc, stage| {
        let stage1 = stage[0];
        let stage2 = stage[1];

        if acc == "".to_string() {
            return format!("\"{}\" -> \"{}\";", stage1, stage2);
        }
        format!("{}\"{}\" -> \"{}\";", acc, stage1, stage2)
    });
    format!(r#"digraph {{ {} }}"#, nodes)
}

fn stage_graphs(buffer: &str) -> Result<Vec<&str>, PestError<Rule>> {
    fn get_stage_names(pairs: Pairs<Rule>) -> impl Iterator<Item = &str> + '_ {
        pairs.flat_map(|stage| {
            if let Rule::stage = stage.as_rule() {
                if let Some(stage_name_span) = stage.into_inner().next() {
                    let stage_name = stage_name_span.as_str();
                    return Some(&stage_name[1..stage_name.len() - 1]);
                }
                return None;
            }
            return None;
        })
    }

    if !is_declarative(buffer) {
        return Err(PestError::new_from_pos(
            ErrorVariant::CustomError {
                message: "The buffer does not appear to be a Declarative Pipeline, I couldn't find pipeline { }".to_string(),
            },
            pest::Position::from_start(buffer),
        ));
    }

    let parser = PipelineParser::parse(Rule::pipeline, buffer)?;
    if let Some(a) = parser
        .flat_map(|parsed| match parsed.as_rule() {
            Rule::stagesDecl => Some(get_stage_names(parsed.into_inner())),
            _ => None,
        })
        .next()
    {
        return Ok(a.collect::<Vec<_>>());
    } else {
        return Err(PestError::new_from_pos(
            ErrorVariant::CustomError {
                message: "I couldn't find stages { }".to_string(),
            },
            pest::Position::from_start(buffer),
        ));
    }
}

pub fn parse_pipeline_string(buffer: &str) -> Result<(), PestError<Rule>> {
    if !is_declarative(buffer) {
        return Err(PestError::new_from_pos(
            ErrorVariant::CustomError {
                message: "The buffer does not appear to be a Declarative Pipeline, I couldn't find pipeline { }".to_string(),
            },
            pest::Position::from_start(buffer),
        ));
    }

    let mut parser = PipelineParser::parse(Rule::pipeline, buffer)?;
    let mut agents = false;
    let mut stages = false;

    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::agentDecl => {
                if agents {
                    return Err(PestError::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Cannot have two top-level `agent` directives".to_string(),
                        },
                        parsed.as_span(),
                    ));
                }
                agents = true;
            }
            Rule::stagesDecl => {
                if stages {
                    return Err(PestError::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Cannot have two top-level `stages` directives".to_string(),
                        },
                        parsed.as_span(),
                    ));
                }
                stages = true;
                parse_stages(&mut parsed.into_inner())?;
            }
            _ => {}
        }
    }
    /*
     * Both agents and stages are required, the lack thereof is an error
     */
    if !agents || !stages {
        let error = PestError::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![],
                negatives: vec![],
            },
            pest::Position::from_start(buffer),
        );
        return Err(error);
    }

    Ok(())
}

/**
 * Run a quick sanity check to determine whether the given buffer appears to
 * be a Declarative Pipeline or not.
 */
fn is_declarative(buffer: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"pipeline(\s+)?\{").expect("Failed to make regex");
    }
    RE.is_match(buffer)
}

/**
 * Make sure that the stage has the required directives, otherwise throw
 * out a CustomError
 */
fn parse_stage(parser: &mut Pairs<Rule>, span: pest::Span) -> Result<(), PestError<Rule>> {
    let mut met_requirements = false;

    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::stepsDecl => {
                met_requirements = true;
            }
            Rule::parallelDecl => {
                met_requirements = true;
            }
            Rule::stagesDecl => {
                met_requirements = true;
                parse_stages(&mut parsed.into_inner())?;
            }
            _ => {}
        }
    }

    if !met_requirements {
        Err(PestError::new_from_span(
            ErrorVariant::CustomError {
                message: "A stage must have either steps{}, parallel{}, or nested stages {}"
                    .to_string(),
            },
            span,
        ))
    } else {
        Ok(())
    }
}

fn parse_stages(parser: &mut Pairs<Rule>) -> Result<(), PestError<Rule>> {
    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::stage => {
                let span = parsed.as_span();
                parse_stage(&mut parsed.into_inner(), span)?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * This will test what parse_pipeline_string will do when faced with a
     * Scripted Pipeline/
     */
    #[test]
    fn is_declarative_with_scripted() {
        assert_eq!(false, is_declarative("node { sh 'env' }"));
    }

    #[test]
    fn is_declarative_with_declarative() {
        assert!(is_declarative(
            "pipeline { agent any stages { stage('Build') { steps { sh 'printenv' } } } }"
        ));
    }

    /*
     * This is just to help make sure the regex isn't too whitespace sensitive
     */
    #[test]
    fn is_declarative_with_declarative_no_spaces() {
        assert!(is_declarative(
            "pipeline{ agent any stages { stage('Build') { steps { sh 'printenv' }}}}"
        ));
    }

    #[test]
    fn parse_string_single() {
        let _str = PipelineParser::parse(Rule::string, r#"'hello world'"#)
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn parse_string_double() {
        let _str = PipelineParser::parse(Rule::string, r#""hello world""#)
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn simple_validation() {
        let _pipeline = PipelineParser::parse(
            Rule::pipeline,
            r#"#!/usr/bin/env groovy

pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh 'ls -lah'
            }
        }
    }
}
"#,
        )
        .expect("Failed to parse")
        .next()
        .expect("Failed to iterate");
    }

    #[test]
    fn parse_no_options() {
        let _options = PipelineParser::parse(Rule::optionsDecl, "options { }")
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn parse_options_no_args() {
        let _options = PipelineParser::parse(Rule::optionsDecl, "options { timestamps() }")
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn parse_options_kwargs() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { timeout(time: 4, unit: 'HOURS') }",
        )
        .unwrap()
        .next()
        .unwrap();
    }

    /*
     * WHY DOES THIS SYNTAX EXIST
     *
     * So annoying. "Declarative"
     */
    #[test]
    fn parse_options_nested_func() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { buildDiscarder(logRotator(daysToKeepStr: '10')) }",
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_options_optional_parens() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { buildDiscarder logRotator(daysToKeepStr: '10') }",
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_triggers() {
        let _t = PipelineParser::parse(Rule::triggersDecl, "triggers { pollSCM('H * * * *') }")
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn parse_environment() {
        let _e = PipelineParser::parse(
            Rule::environmentDecl,
            r#"environment {
                DISABLE_PROXY_CACHE = 'true'
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_block_steps() {
        let _s = PipelineParser::parse(Rule::step, "dir('foo') { sh 'make' }")
            .unwrap()
            .next()
            .unwrap();
    }

    #[test]
    fn parse_complex_step() {
        let _s = PipelineParser::parse(
            Rule::step,
            r#"checkout([
                $class: 'GitSCM',
                branches: [
                    [name: "refs/heads/${env.BRANCH_NAME}"]
                ],
                gitTool: scm.gitTool,
                extensions: [
                    [name: "refs/heads/${env.BRANCH_NAME}"],
                ],
            ])"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_not_exactly_declarative_is_it_step() {
        let _s = PipelineParser::parse(
            Rule::step,
            r#"checkout([
                $class: 'GitSCM',
                userRemoteConfigs: [
                    [ refspec: scm.userRemoteConfigs[0].refspec,
                      url: scm.userRemoteConfigs[0].url
                    ]
                ],
            ])"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_steps_with_triple_singles() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                sh '''
                    env
                '''
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_steps_with_triple_doubles() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                sh """
                    env
                """
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    /*
     * I kind of cannot believe that this is legitimate Declarative but it
     * apparently is!
     */
    #[test]
    fn parse_string_with_concatenation() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                echo 'Hello world: ' + WORKSPACE
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_step_with_symbol_concatenation() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                ws(dir: WORKSPACE + '/foo') {
                    sh 'pwd'
                }
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_steps_with_parens() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                deleteDir()
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_script_step() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                script {
                    def taskOutput = readJSON file: 'task-output.dev.json'
                    def revision = taskOutput.taskDefinition.revision
                    sh "aws ecs update-service --cluster ${CLUSTER} --service ${SERVICE} --task-definition ${FAMILY}:${revision}"
                }
            }"#)
        .unwrap().next().unwrap();
    }

    #[test]
    fn parse_script_step_nesting() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                script {
                    withAnt(installation: 'ant-latest') {
                        if (isUnix()) {
                            sh 'ant info'
                        }
                        else {
                            bat 'ant info'
                        }
                    }
                }
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    /*
     * I put a step in your step so you can step while you step
     */
    #[test]
    fn parse_sup_dawg_heard_you_liked_steps() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                sh 'rm -f task-definition.*.json'

                writeJSON(file: 'task-definition.dev.json',
                        json: readYaml(text: readFile('deploy/task-definition.yml')))
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_abusive_chaining_of_groovy_on_steps() {
        let _s = PipelineParser::parse(
            Rule::stepsDecl,
            r#"steps {
                sh 'rm -f task-definition.*.json'

                writeJSON(file: 'task-definition.dev.json',
                        json: readYaml(text: readFile('deploy/task-definition.yml')
                                                    .replaceAll('@@IMAGE@@', params.IMAGE)
                                                    .replaceAll('@@FAMILY@@', params.FAMILY)))
                sh 'echo DEV task definition:'
                sh 'cat task-definition.dev.json'
            }"#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    /*
     * More wacky but actually valid code
     */
    #[test]
    fn parse_environment_with_embedded_steps() {
        let _s = PipelineParser::parse(
            Rule::environmentDecl,
            r#"environment {
                // Using returnStdout
                CC = """${sh(
                        returnStdout: true,
                        script: 'echo "clang"'
                    )}"""
                }
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn test_stage_graphs() {
        assert_eq!(
            stage_graphs("pipeline{ agent any stages { stage('Build') { steps { sh 'printenv' }} stage('Test') { steps { sh 'cargo test' } } }}").unwrap(),
            vec!["Build", "Test"]
        )
    }

    #[test]
    fn test_build_dot_file() {
        assert_eq!(
            build_dot_file(stage_graphs("pipeline{ agent any stages { stage('Build') { steps { sh 'printenv' }} stage('Test') { steps { sh 'cargo test' } } stage('Publish') { steps { sh 'cargo publish' } } }}").unwrap()),
            r#"digraph { "Build" -> "Test";"Test" -> "Publish"; }"#
        )
    }
}
