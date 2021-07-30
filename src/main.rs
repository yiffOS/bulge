mod commands;
mod util;

use std::env;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if any command was supplied
    if args.len() < 2 {
        commands::help::help();
        std::process::exit(0);
    }

    let command: String = args[1].to_lowercase();

    match &command[..] {
        // Help commands
        "-h" => commands::help::help(),
        "--help" => commands::help::help(),

        // Show help if invalid command is given
        _ => commands::help::help()
    }
}
