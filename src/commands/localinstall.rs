use std::path::Path;
use std::fs;
use text_io::read;

use crate::util::lock::{create_lock, remove_lock, lock_exists};
use crate::util::packaging::fns::{check_if_package, decompress_xz, decode_pkg_file};

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

            let mut package_tar = decompress_xz(fs::File::open(i).expect("Failed to read package!"));

            package_tar.unpack(format!("/tmp/bulge/{}", i.split("/").last().unwrap())).unwrap();

            let package = decode_pkg_file(fs::File::open(format!("/tmp/bulge/{}/PKG", i.split("/").last().unwrap()))
                                                    .expect("Failed to open PKG file!"));

            println!("Installing package {} v{} from {}.", package.name, package.version, &i);
            println!("Continue? [y/N]");
            let s: String = read!();
            if !(s.to_lowercase() == "y".parse::<String>().unwrap()) {
                println!("Abandoning install!");
                std::process::exit(1);
            }

            
        }
    }

    remove_lock().expect("Failed to remove lock?");
}