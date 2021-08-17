mod commands;
mod util;

use std::env;
use xdg::BaseDirectories;
use crate::util::mirrors::load_mirrors;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn get_xdg_direct() -> BaseDirectories {
    BaseDirectories::with_prefix("bulge").expect("Error getting XDG base directories")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let gaming = load_mirrors();

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

        // Sync commands
        "s" => commands::sync::sync(),
        "sync" => commands::sync::sync(),

        // Upgrade commands
        "u" => commands::upgrade::upgrade(),
        "upgrade" => commands::upgrade::upgrade(),

        // Install commands
        "i" => commands::install::install(args),
        "install" => commands::install::install(args),

        // Remove commands

        // Info commands

        // List commands

        // Show help if invalid command is given
        _ => commands::help::help()
    }
}
