use std::io::Read;
use std::{vec, fs};
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
        if Path::new(i).exists() {
            // Decompress the tar file
            let mut contents = Vec::new();
            let file_bytes = fs::read(i).expect("Failed to read package!");
            let mut decompressor = XzDecoder::new(file_bytes.as_slice());

            decompressor.read(&mut contents).expect("Failed to decompress package!");

            let mut a = Archive::new(contents.as_slice());

            // Look for PKG file
            for file in a.entries().unwrap() {
                let mut file = file.expect("I/O Error!");

                println!("{:?}", file.header().path().unwrap());
            }
        }
    }

    remove_lock().expect("Failed to remove lock?");
}