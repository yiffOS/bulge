use std::path::Path;
use crate::util::database::fns::get_conflicts;
use crate::util::database::structs::InstalledPackages;

pub struct ConflictingFiles {
    pub is_conflict: bool,
    pub files: Vec<String>,
}

pub struct ConflictingPackages {
    pub is_conflict: bool,
    pub packages: Vec<InstalledPackages>,
}

pub fn run_conflict_check(files: &Vec<String>, is_installed: bool, root: String) -> ConflictingFiles {
    let mut conflicting_struct = ConflictingFiles {
        is_conflict: false,
        files: vec![]
    };

    for i in files {
        if !is_installed && Path::new(format!("{}{}", &root, &i).as_str()).exists() {
            conflicting_struct.is_conflict = true;
            conflicting_struct.files.push(format!("{}{}", root.clone(), i.clone()));
        }
    }

    return conflicting_struct;
}

pub fn run_conflict_package_check(package: &String) -> ConflictingPackages {
    let mut conflicting_struct = ConflictingPackages {
        is_conflict: false,
        packages: vec![]
    };

    for i in get_conflicts(package) {
        if &i.name == package {
            // Whoops we found a conflict with ourselves, lets skip this one
            // TODO: Find a better way to search in sqlite so this isn't needed
            continue;
        }

        conflicting_struct.is_conflict = true;
        conflicting_struct.packages.push(i);
    }

    return conflicting_struct;
}