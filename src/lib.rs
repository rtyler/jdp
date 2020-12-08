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
}
