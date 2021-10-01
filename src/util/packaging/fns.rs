use std::fs::File;

use xz2::read::XzDecoder;
use tar::Archive;
use std::path::Path;

use crate::util::packaging::structs::Package;

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