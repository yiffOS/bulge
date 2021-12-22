use std::fs;
use std::fs::File;
use std::path::Path;
use version_compare::Version;
use crate::util::database::fns::{add_package_to_installed, get_installed_package};
use crate::util::database::structs::Source;
use crate::util::lock::remove_lock;
use crate::util::macros::{continue_prompt, get_root, string_to_vec};
use crate::util::packaging::fns::{decode_pkg_file, decompress_xz};
use crate::util::packaging::structs::{NewPackage, Package};
use crate::util::transactions::conflict::run_conflict_check;

#[derive(PartialEq, Eq, Hash, Clone)]
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

    // Check if package is already installed
    let installed_pkg = get_installed_package(&package.name);
    let mut reinstall = false;
    if installed_pkg.is_ok() {
        // Check if this is a downgrade
        if Version::from(&package.version) < Version::from(&installed_pkg.as_ref().unwrap().version) {
            let installed_pkg = get_installed_package(&package.name); // Result doesn't have copy

            // Ask the user if they'd like to still install the specified package
            println!("> This will result in a downgrade as {} v{} is already installed!", &package.name, &installed_pkg.unwrap().version);

            let s = continue_prompt();

            if !s {
                println!("> Abandoning install!");

                remove_lock().expect("Failed to remove lock!");
                std::process::exit(1);
            }
        } else if Version::from(&package.version) == Version::from(&installed_pkg.as_ref().unwrap().version) {
            println!("> Warning: {} is already installed, reinstalling...", &package.name);
        }

        reinstall = true;
    }

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

    let conflicting = run_conflict_check(&files, installed_pkg.is_ok(), get_root());

    if conflicting.is_conflict {
        eprintln!("Package files already exist on the file system!");

        let s = continue_prompt();

        if !s {
            println!("Abandoning install!");

            remove_lock().expect("Failed to remove lock?");

            std::process::exit(1);
        } else {
            println!("Continuing install!");

            for i in conflicting.files {
                println!("Removing {}", i);
                if Path::new(&i).exists() {
                    fs::remove_file(&i).expect("Failed to delete file!");
                }
            }
        }
    }

    if reinstall {
        for i in &files {
            if Path::new(&i).exists() {
                fs::remove_file(&i).expect("Failed to delete file!");
            }
        }
    }

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

    //Add package to database
    add_package_to_installed(NewPackage {
        name: install.package.name.clone(),
        groups: install.package.groups,
        version: install.package.version.clone(),
        epoch: install.package.epoch,
        installed_files: files,
        provides: string_to_vec(install.package.provides),
        conflicts: string_to_vec(install.package.conflicts),
        dependencies: string_to_vec(install.package.depends),
    }, install.source);
}