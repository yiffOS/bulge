use std::collections::{HashMap, HashSet};
use crate::util::database::fns::{get_dependencies, get_installed_package};
use crate::util::macros::string_to_vec;
use crate::util::packaging::structs::Package;

/// Returns a list of all dependencies for a given package.
pub fn run_depend_resolve(package: Package) -> HashSet<String> {
    let mut dependencies = HashSet::new();

    if package.depends.is_empty() {
        return dependencies;
    }

    for dep in string_to_vec(package.depends) {
        // Insert top level dependency into set
        dependencies.insert(dep.clone());

        // Check database to get dependencies of dependency
        for deeper_dep in get_dependencies(dep.clone()) {
            // Insert depend we're looking into in case it doesn't have any dependencies
            dependencies.insert(deeper_dep.name.clone());

            dependencies.extend(run_depend_resolve(deeper_dep));
        }
    }

    return dependencies;
}

/// Check to see if provided dependencies are installed.
pub fn run_depend_check(packages: HashSet<String>) -> HashMap<String, bool> {
    let mut dependencies = HashMap::new();

    for package in packages.iter() {
        if get_installed_package(&package).is_ok() {
            dependencies.insert(package.clone(), true);
        } else {
            dependencies.insert(package.clone(), false);
        }
    }

    return dependencies;
}