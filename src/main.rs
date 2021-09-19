mod commands;
mod util;
mod static_files;

use std::env;
use xdg::BaseDirectories;

/// Get a static string of the current bulge version
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get a XDG base directory
pub fn get_xdg_direct() -> BaseDirectories {
    BaseDirectories::with_prefix("bulge").expect("Error getting XDG base directories")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
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

        // Sync commands
        "s" => commands::sync::sync().await,
        "sync" => commands::sync::sync().await,

        // Upgrade commands
        "u" => commands::upgrade::upgrade(),
        "upgrade" => commands::upgrade::upgrade(),

        // Install commands
        "i" => commands::install::install(args),
        "install" => commands::install::install(args),

        // Group install commands

        // Remove commands

        // Info commands

        // List commands

        // Internal commands for setup
        "setup" => commands::setup::init().await,

        // Specify that command is invalid and show help command
        _ => {
            println!("bulge: Invalid command \"{}\", use {{-h --help}} for valid commands.", command);
        }
    }
}
