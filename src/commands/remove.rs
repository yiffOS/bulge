use std::collections::HashSet;

use crate::util::{lock::{create_lock, lock_exists, remove_lock}, packaging::fns::run_remove};
use crate::util::database::fns::get_installed_package;
use crate::util::database::structs::InstalledPackages;
use crate::util::macros::{continue_prompt, display_removing_packages};

pub fn remove(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a path to a package to remove. (Check bulge --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");
    lock_exists();
    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let packages: HashSet<String> = args.clone().drain(2..).collect();
    let mut removal_queue: HashSet<InstalledPackages> = HashSet::new();

    println!("==> Collecting removal queue...");
    for i in packages {
        let installed_pkg = get_installed_package(&i);

        if installed_pkg.is_ok() {
            removal_queue.insert(installed_pkg.unwrap());
        } else {
            eprintln!("=> Package {} is not installed. Skipping...", i);
        }
    }

    println!("==> Checking for dependencies...");
    for i in removal_queue.clone() {

    }

    println!("\nPackages to remove [{}]: {}\n", removal_queue.len(), display_removing_packages(removal_queue.clone()));

    if !continue_prompt() {
        println!("\n==> Aborting!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(0);
    }

    println!("\n==> Removing packages...");

    for i in removal_queue {
        println!("=> Removing {} {}-{}...", &i.name, &i.version, &i.epoch);
        run_remove(&i.name);
    }

    println!("\n==> Complete!");

    remove_lock().expect("Failed to remove lock");
}