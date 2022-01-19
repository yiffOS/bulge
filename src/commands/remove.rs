use std::collections::{HashMap, HashSet};

use crate::util::{lock::{create_lock, lock_exists, remove_lock}, packaging::fns::run_remove};
use crate::util::database::fns::{get_depended_on, get_installed_package};
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

    println!("==> Getting packages...");
    let raw_packages: Vec<String> = args.clone().drain(2..).collect();
    let mut packages: HashSet<InstalledPackages> = HashSet::new();

    for i in raw_packages {
        let package = get_installed_package(&i);

        if package.is_ok() {
            packages.insert(package.unwrap());
        } else {
            println!("WARN> Package {} not found.", i);
        }
    }

    if packages.is_empty() {
        println!("ERR> No valid packages specified!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(1);
    }

    println!("==> Checking dependencies...");
    let mut abort = false;
    let mut abort_map: HashMap<InstalledPackages, Vec<InstalledPackages>> = HashMap::new();
    for i in packages.clone() {
        let mut abort_vec: Vec<InstalledPackages> = Vec::new();

        for x in get_depended_on(&i.name) {
            abort = true;
            abort_vec.push(x);
        }

        abort_map.insert(i, abort_vec);
    }

    if abort {
        println!("ERR> The following packages are depended on by other packages:");
        for (i, v) in abort_map.iter() {
            println!("{} {}-{} is required by:", i.name, i.version, i.epoch);
            for x in v {
                println!("\t{} {}-{}", x.name, x.version, x.epoch);
            }
        }

        println!("ERR> Please remove the above packages before continuing.");
        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(1);
    }

    println!("\nPackages to remove [{}]: {}\n", packages.len(), display_removing_packages(packages.clone()));

    if !continue_prompt() {
        println!("\n==> Aborting!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(0);
    }

    println!("\n==> Removing packages...");

    for i in packages {
        println!("=> Removing {} {}-{}...", &i.name, &i.version, &i.epoch);
        run_remove(&i.name);
    }

    println!("\n==> Complete!");

    remove_lock().expect("Failed to remove lock");
}