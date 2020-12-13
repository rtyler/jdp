extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error as PestError;
use std::path::PathBuf;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
pub struct PipelineParser;

pub fn parse_file(path: &PathBuf) -> Result<(), pest::error::Error<Rule>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).expect(&format!("Failed to open {:?}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file into string");

    parse_pipeline_string(&contents)
}

pub fn parse_pipeline_string(buffer: &str) -> Result<(), PestError<Rule>> {
    let _parser = PipelineParser::parse(Rule::pipeline, buffer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_single() {
        let _str = PipelineParser::parse(
            Rule::string,
            r#"'hello world'"#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_string_double() {
        let _str = PipelineParser::parse(
            Rule::string,
            r#""hello world""#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn simple_validation() {
        let _pipeline = PipelineParser::parse(
            Rule::pipeline,
            r#"
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
"#)
        .expect("Failed to parse")
        .next()
        .expect("Failed to iterate");
    }

    #[test]
    fn parse_no_options() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { }")
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_options_no_args() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { timestamps() }")
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_options_kwargs() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { timeout(time: 4, unit: 'HOURS') }")
            .unwrap().next().unwrap();
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
            "options { buildDiscarder(logRotator(daysToKeepStr: '10')) }")
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_options_optional_parens() {
        let _options = PipelineParser::parse(
            Rule::optionsDecl,
            "options { buildDiscarder logRotator(daysToKeepStr: '10') }")
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_triggers() {
        let _t = PipelineParser::parse(
            Rule::triggersDecl,
            "triggers { pollSCM('H * * * *') }")
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_environment() {
        let _e = PipelineParser::parse(
            Rule::environmentDecl,
            r#"environment {
                DISABLE_PROXY_CACHE = 'true'
            }"#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_block_steps() {
        let _s = PipelineParser::parse(
            Rule::step,
            "dir('foo') { sh 'make' }")
            .unwrap().next().unwrap();
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
            ])"#)
            .unwrap().next().unwrap();
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
            ])"#)
            .unwrap().next().unwrap();
    }

}
