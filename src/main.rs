extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
struct PipelineParser;

fn main() {
    println!("Hello, world!");
}
pub use pest::error::ErrorVariant;
pub use pest::error::LineColLocation;


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
