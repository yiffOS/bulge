use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::util::database::structs::Source;
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::{continue_prompt, get_root};
use crate::util::packaging::fns::{check_if_package, decode_pkg_file, decompress_xz};
use crate::util::transactions::install::{InstallTransaction, run_install};

pub fn local_install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a path to a package to install. (Check bulge --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");
    lock_exists();
    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let packages: Vec<String> = args.clone().drain(2..).collect();

    println!("==> Resolving packages...");
    let mut package_queue: HashMap<InstallTransaction, File> = HashMap::new();
    for i in &packages {
        // Check if i is a valid path and assume it's a file we want to install if it is
        if Path::new(i).exists() {
            if !check_if_package(decompress_xz(fs::File::open(i).expect("Failed to read package!"))) {
                println!("WARN> {} is not a valid package!", i);
            }

            let mut package_tar = decompress_xz(fs::File::open(i).expect("Failed to read package!"));
            package_tar.unpack(format!("{}/tmp/bulge/{}", get_root(), &i)).unwrap();

            let package = decode_pkg_file(fs::File::open(format!("{}/tmp/bulge/{}/PKG", get_root(), &i))
                .expect("Failed to open PKG file!"));

            package_queue.insert(InstallTransaction {
                package: package,
                source: Source{ name: "local".to_string(), url: None }
            }, fs::File::open(i).expect("Failed to read package!"));
        } else {
            println!("WARN> {} is not a valid package!", i);
        }
    }

    let mut temp_string = String::new();

    if package_queue.is_empty() {
        println!("ERR> No packages to install!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(1);
    }

    for (i, _f) in &package_queue {
        temp_string.push_str(&*i.package.name);
        temp_string.push_str("-");
        temp_string.push_str(&*i.package.version);
        temp_string.push_str("-");
        temp_string.push_str(&*i.package.epoch.to_string());
        temp_string.push_str(" ");
    }

    println!("\nPackages to install [{}]: {}\n", &package_queue.len(), temp_string);

    if !(continue_prompt()) {
        println!("Abandoning install!");

        remove_lock().expect("Failed to remove lock?");
        std::process::exit(1);
    }

    println!("\n==> Installing packages...");
    let mut clean_up_list: Vec<String> = Vec::new();
    for (i, f) in package_queue {
        println!("=> Installing {} v{}-{}...", &i.package.name, &i.package.version, &i.package.epoch);

        run_install(i.clone(), f);

        clean_up_list.push(i.package.name.clone());
    }

    println!("\n==> Cleaning up...");

    for i in &packages {
        fs::remove_dir_all(format!("{}/tmp/bulge/{}", get_root(), &i))
            .expect("Failed to delete temp path!");
    }

    for i in clean_up_list {
        fs::remove_dir_all(format!("{}/tmp/bulge/{}", get_root(), &i))
            .expect("Failed to delete temp path!");
    }

    println!("\n==> Complete!");

    remove_lock().expect("Failed to remove lock?");
}