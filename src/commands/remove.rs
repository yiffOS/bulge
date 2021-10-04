use text_io::read;

use crate::util::{lock::{create_lock, lock_exists, remove_lock}, packaging::fns::run_remove};

pub fn remove(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a path to a package to remove. (Check bulge --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let packages: Vec<String> = args.clone().drain(2..).collect();

    println!("This operation will uninstall {:?}, are you sure?", packages);
    println!("Continue? [y/N]");
    let s: String = read!();
    if !(s.to_lowercase() == "y".parse::<String>().unwrap()) {
        println!("Abandoning removal!");
        std::process::exit(1);
    }

    for i in packages {
        println!("Removing {}...", &i);
        run_remove(&i);
    }

    println!("Complete!");

    remove_lock().expect("Failed to remove lock");
}