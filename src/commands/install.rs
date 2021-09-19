use std::vec;

use crate::util::lock::{create_lock, remove_lock, lock_exists};
use crate::util::database::fns::search_for_package;

pub fn install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a package to install. (Check bulge --help for usage)");

        remove_lock().expect("Failed to remove lock?");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let packages: Vec<String> = args.clone().drain(2..).collect();
    let mut repos: Vec<String> = vec![];

    for i in &packages {
        let repo: String = search_for_package(i);
    
        if repo.is_empty() {
            eprintln!("{} was not found!", i);
    
            remove_lock().expect("Failed to remove lock?");
    
            std::process::exit(1);
        }

        repos.push(repo);
    }

    // TODO: These are debug messages, remove it once function is complete
    println!("Amount of packages: {}", packages.len());
    println!("Amount of repos: {}", repos.len());
    println!("Package vec: {:?}", packages);
    println!("Repo vec: {:?}", repos);

    remove_lock().expect("Failed to remove lock?");
}