use std::path::Path;
use std::fs;

use crate::util::lock::{create_lock, remove_lock, lock_exists};
use crate::util::packaging::fns::{check_if_package, decompress_xz, run_install};

pub fn local_install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a path to a package to install. (Check bulge --help for usage)");

        remove_lock().expect("Failed to remove lock?");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let packages: Vec<String> = args.clone().drain(2..).collect();

    for i in &packages {
        // Check if i is a valid path and assume it's a file we want to install if it is
        if Path::new(i).exists() {
            if !check_if_package(decompress_xz(fs::File::open(i).expect("Failed to read package!"))) {
                eprintln!("{} is not a valid package!", i);
    
                remove_lock().expect("Failed to remove lock?");
        
                std::process::exit(1);
            }

            run_install(fs::File::open(i).expect("Failed to read package!") ,i.split("/").last().unwrap(),i);
        }
    }
    remove_lock().expect("Failed to remove lock?");
}