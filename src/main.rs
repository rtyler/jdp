use jdp::*;

use gumdrop::Options;

// Define options for the program.
#[derive(Debug, Options)]
struct JdpOptions {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "be verbose")]
    verbose: bool,
    #[options(command)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "Validate the syntax of a Jenkinsfile")]
    Check(CheckOpts),
}

// Options accepted for the `make` command
#[derive(Debug, Options)]
struct CheckOpts {
    #[options(help = "print help message")]
    help: bool,
    #[options(free, required, help = "Path to a Jenkinsfile")]
    file: std::path::PathBuf,
}

/// The number of lines of context to show for errors
const LINES_OF_CONTEXT: usize = 4;

fn main() {
    pretty_env_logger::init();
    let opts = JdpOptions::parse_args_default_or_exit();

    if opts.command.is_none() {
        println!("You must specify a command!");
        return;
    }

    match opts.command.unwrap() {
        Command::Check(checkopts) => {
            if let Err(error) = parse_file(&checkopts.file) {
                use pest::error::ErrorVariant;
                use pest::error::LineColLocation::{Pos, Span};
                use std::fs::File;
                use std::io::{BufRead, BufReader};

                if checkopts.file.is_file() {
                    let file = File::open(&checkopts.file).expect("Failed to reopen file");
                    let lines: Vec<String> =
                        BufReader::new(file).lines().map(|l| l.unwrap()).collect();

                    let filename = checkopts.file.as_path().to_string_lossy();
                    println!("\n{}", filename);
                    for _ in 0..filename.len() {
                        print!("-");
                    }
                    println!("");

                    match error.line_col {
                        Pos((line, column)) => {
                            let start_line = if line < LINES_OF_CONTEXT {
                                0
                            } else {
                                line - LINES_OF_CONTEXT
                            };

                            for n in start_line..line {
                                println!("{}: {}", n, lines[n]);
                            }
                            // Just a little spacer for the error
                            print!("  ");
                            for _ in 0..column {
                                print!("-");
                            }
                            println!("^");
                        }
                        Span(start, end) => {
                            let start_line = if start.0 < LINES_OF_CONTEXT {
                                0
                            } else {
                                start.0 - LINES_OF_CONTEXT
                            };
                            for n in start_line..start.0 {
                                println!("{}: {}", n, lines[n]);
                            }
                            // Just a little spacer for the error
                            print!("  ");
                            for _ in 0..(end.1 - 2) {
                                print!("-");
                            }
                            println!("^");
                        }
                    }
                }

                match error.variant {
                    ErrorVariant::CustomError { message } => {
                        println!("\nFail: {}", message);
                    }
                    _ => {
                        println!("\nFailed to parse: missing required syntax");
                    }
                }
                std::process::exit(1);
            } else {
                println!("Looks valid! Great work!");
            }
        }
    }
}
