use std::fs;
use std::fs::File;
use crate::util::database::fns::add_package_to_installed;
use crate::util::database::structs::Source;
use crate::util::macros::get_root;
use crate::util::packaging::fns::{decode_pkg_file, decompress_xz};
use crate::util::packaging::structs::{NewPackage, Package};

#[derive(PartialEq, Eq, Hash)]
pub struct InstallTransaction {
    pub package: Package,
    pub source: Source,
}

pub fn run_install(install: InstallTransaction, file: File) {
    let mut package_tar = decompress_xz(file);

    package_tar.unpack(format!("{}/tmp/bulge/{}", get_root(), &install.package.name))
        .expect("Failed to unpack package");

    let package = decode_pkg_file(fs::File::open(format!("{}/tmp/bulge/{}/PKG", get_root(), &install.package.name))
        .expect("Failed to open PKG file!"));

    // Decompress data
    let mut data_tar_files = decompress_xz(
        fs::File::open(
            format!("{}/tmp/bulge/{}/data.tar.xz", get_root(), &install.package.name)
        ).expect("Failed to read package!")
    );

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

    //Add package to database
    add_package_to_installed(NewPackage {
        name: install.package.name.clone(),
        groups: install.package.groups,
        version: install.package.version.clone(),
        epoch: install.package.epoch,
        installed_files: files
    }, install.source);

    // Open data tar for extraction
    let mut data_tar = decompress_xz(
        fs::File::open(
            format!("{}/tmp/bulge/{}/data.tar.xz", get_root(), &install.package.name)
        ).expect("Failed to read package!")
    );

    // Extract files onto root
    data_tar.set_preserve_permissions(true);
    data_tar.set_unpack_xattrs(true);

    data_tar
        .unpack(get_root() + "/")
        .expect("Extraction error!");
}