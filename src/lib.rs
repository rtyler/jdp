extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error as PestError;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
pub struct PipelineParser;

pub fn parse_pipeline_string(buffer: &str) -> Result<(), PestError<Rule>> {
    let mut parser = PipelineParser::parse(Rule::pipeline, buffer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
