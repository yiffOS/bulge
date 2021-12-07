use std::{fs::{self, File}, vec};

use text_io::read;
use version_compare::Version;
use xz2::read::XzDecoder;
use tar::Archive;
use std::path::Path;

use crate::util::{database::{fns::{add_package_to_installed, get_installed_package, remove_package_from_installed, return_owned_files}, structs::Source}, lock::remove_lock, packaging::structs::{NewPackage, Package}};

pub fn decompress_xz(compressed_tar: File) -> Archive<XzDecoder<File>> {
    return Archive::new(XzDecoder::new(compressed_tar));
}

pub fn decode_pkg_file(pkg: File) -> Package {
    let v: Package = serde_json::from_reader(pkg).unwrap();

    return v;
}

pub fn check_if_package(mut xztar: Archive<XzDecoder<File>>) -> bool {    
    // Look for PKG file
    for file in xztar.entries().unwrap() {
        if file.unwrap().header().path().unwrap() == Path::new("PKG") {
            // If a PKG file is found then this is a valid package
            return true;
        }                
    }

    return false;
}

pub fn run_install(file: File, tmp_path: &str, source: Source) {
    let mut package_tar = decompress_xz(file);

    package_tar.unpack(format!("/tmp/bulge/{}", tmp_path)).unwrap();

    let package = decode_pkg_file(fs::File::open(format!("/tmp/bulge/{}/PKG", tmp_path))
        .expect("Failed to open PKG file!"));

    // TODO: Check for conflicting packages
    println!("Looking for conflicting packages...");

    // TODO: Check for dependencies
    println!("Looking for dependencies...");

    println!();
    println!("Installing package {} v{} from {}.", &package.name, &package.version, &source.name);

    // Ask the user if they'd like to install the specified package
    println!("Continue? [y/N]");
    
    let s: String = read!();
    if !(s.to_lowercase() == "y".parse::<String>().unwrap()) {
        println!("Abandoning install!");
        std::process::exit(1);
    }

    // Check if package is already installed
    let installed_pkg = get_installed_package(&package.name);
    if installed_pkg.is_ok() {
        // Check if this is a downgrade
        if Version::from(&package.version) > Version::from(&installed_pkg.as_ref().unwrap().version) {
            let installed_pkg = get_installed_package(&package.name); // Result doesn't have copy

            // Ask the user if they'd like to still install the specified package
            println!("This will result in a downgrade as {} v{} is already installed!", &package.name, &installed_pkg.unwrap().version);

            println!("Continue? [y/N]");
            let s: String = read!();
            if !(s.to_lowercase() == "y".parse::<String>().unwrap()) {
                println!("Abandoning install!");
                std::process::exit(1);
            }
        }

        println!("Warning: {} is already installed, reinstalling...", &package.name);
    }

    // Decompress data
    let mut data_tar_files = decompress_xz(fs::File::open(format!("/tmp/bulge/{}/data.tar.xz", tmp_path)).expect("Failed to read package!"));

    // Calculate files to be installed and extract to temp folder
    let mut files: Vec<String> = vec![];

    data_tar_files.entries()
        .expect("IO Error!")
        .filter_map(|e| e.ok())
        .for_each(|x| {
            if !x.header().path().unwrap().to_string_lossy().ends_with("/") {
                files.push(format!("/{}" ,x.header().path().unwrap().to_string_lossy().to_string()));
            }
        });

    println!();
    println!("Looking for conflicting files...");
    let mut conflict = false;
    for i in &files {
        if !installed_pkg.is_ok() && Path::new(&i).exists() {
            println!("{} already exists!", i);
            conflict = true;
        }
    }

    if conflict {
        eprintln!("Package files already exist on the file system!");

        println!("Continue? THIS WILL DELETE FILES! [y/N]");
        let s: String = read!();

        if !(s.to_lowercase() == "y".parse::<String>().unwrap()) {
            println!("Abandoning install!");

            remove_lock().expect("Failed to remove lock?");

            std::process::exit(1);
        } else {
            println!("Continuing install!");

            // Redo the loop but delete files now
            for i in &files {
                if !installed_pkg.is_ok() && Path::new(&i).exists() {
                    println!("DELETING {}!", i);
                    fs::remove_file(i).expect("Failed to delete file!");
                }
            }
        }
    }

    //Add package to database
    add_package_to_installed(NewPackage { 
        name: package.name.clone(), 
        groups: package.groups, 
        version: package.version.clone(), 
        epoch: package.epoch, 
        installed_files: files
    }, source);

    println!("Decompressing files...");
    
    // Open data tar for extraction
    let mut data_tar = decompress_xz(fs::File::open(format!("/tmp/bulge/{}/data.tar.xz", tmp_path)).expect("Failed to read package!"));

    println!("Unpacking files...");

    // Extract files onto root
    data_tar.unpack("/")
        .expect("Extraction error!");
    
    println!();
    println!("Installed {} v{}!", &package.name, &package.version);
}

pub fn run_remove(package: &String) {
    for x in return_owned_files(package).expect("Failed to get owned files!") {
        if Path::new(&x).exists() {
            fs::remove_file(x).expect("Failed to delete file!")
        }
    }

    remove_package_from_installed(package).expect("Failed to remove package from database.");
}