use std::fs;
use std::path::Path;
use xz2::read::XzDecoder;
use tar::Archive;

use crate::util::lock::{create_lock, remove_lock, lock_exists};

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
        println!("Getting package from {}", i);
        if Path::new(i).exists() {
            // Decompress the tar file
            let file_bytes = fs::File::open(i).expect("Failed to read package!");
            let decompressor = XzDecoder::new(file_bytes);

            let mut a = Archive::new(decompressor);

            println!("Decompressed package, looking for package information.");

            // Look for PKG file
            for file in a.entries().unwrap() {
                if  file.unwrap().header().path().unwrap() == Path::new("PKG") {
                    // If a PKG file is found then this is a valid package
                    println!("PKG found!");
                }

                
            }
        }
    }

    remove_lock().expect("Failed to remove lock?");
}