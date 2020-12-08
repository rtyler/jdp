
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
    #[options(free, required, help="Path to a Jenkinsfile")]
    file: std::path::PathBuf,
}

fn main() {
    let opts = JdpOptions::parse_args_default_or_exit();

    if opts.command.is_none() {
        println!("You must specify a command!");
        return;
    }

    match opts.command.unwrap() {
        Command::Check(checkopts) => {
            println!("Checking: {}", checkopts.file.as_path().display());

            let result = parse_file(&checkopts.file);

            if result.is_err() {
                println!("Failed to parse!: {:#?}", result);
                std::process::exit(1);
            }
            else {
                println!("Looks valid! Great work!");
            }
        },
    }
}
