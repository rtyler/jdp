/*
 * This test module will test everything in test_data/valid
 */
use jdp::*;
use glob::glob;

fn test_glob(pattern: &str, can_parse: bool) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                match path.file_name() {
                    Some(name) => {
                        if name == "Jenkinsfile" {
                            let result = parse_file(&path);
                            if can_parse && result.is_err() {
                                println!("{:?}", result);
                            }
                            assert_eq!(can_parse, result.is_ok(), "Parsing file failed {:?}", path);
                        }
                    },
                    _ => {},
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

#[test]
fn test_valid_pipelines() {
    test_glob("data/valid/**/Jenkinsfile", true);
}

#[test]
fn test_invalid_pipelines() {
    test_glob("data/invalid/**/Jenkinsfile", false);
}
