use std::fs::{self, File};
use std::path::Path;

use tar::Archive;
use xz2::read::XzDecoder;

use crate::util::{database::fns::{remove_package_from_installed, return_owned_files}, packaging::structs::Package};

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

pub fn run_remove(package: &String) {
    for x in return_owned_files(package).expect("Failed to get owned files!") {
        if Path::new(&x).exists() {
            fs::remove_file(x).expect("Failed to delete file!")
        }
    }

    remove_package_from_installed(package).expect("Failed to remove package from database.");
}