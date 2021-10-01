use std::fs::{self, File};

use text_io::read;
use version_compare::Version;
use xz2::read::XzDecoder;
use tar::Archive;
use std::path::Path;

use crate::util::{database::fns::get_installed_package, packaging::structs::Package};

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

pub fn run_install(file: File, tmp_path: &str, source: &String) {
    let mut package_tar = decompress_xz(file);

    package_tar.unpack(format!("/tmp/bulge/{}", tmp_path)).unwrap();

    let package = decode_pkg_file(fs::File::open(format!("/tmp/bulge/{}/PKG", tmp_path))
        .expect("Failed to open PKG file!"));

    println!("Installing package {} v{} from {}.", &package.name, &package.version, &source);

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
        if Version::from(&package.version) > Version::from(&installed_pkg.unwrap().version) {
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
    }
}