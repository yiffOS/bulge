use rusqlite::{Connection, params};
use crate::util::{database::structs::Source, macros::{string_to_vec, vec_to_string}, packaging::structs::{NewPackage, Package}};
use std::{time::{SystemTime, UNIX_EPOCH}, vec};
use crate::util::config::fns::get_sources;
use std::{error::Error, fmt};
use std::collections::HashSet;
use crate::util::database::structs::RemotePackage;
use crate::util::macros::get_root;

use super::structs::InstalledPackages;

#[derive(Debug)]
pub struct PackageDBError;

impl Error for PackageDBError {}

impl fmt::Display for PackageDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error getting package from database!")
    }
}

/// Creates a database containing locally installed packages and various information
pub fn init_database() {
    let conn = Connection::open(get_root() + "/etc/bulge/databases/bulge.db").expect("Failed to create package database");

    conn.execute(
        "create table if not exists installed_packages
            (
                name text not null unique primary key,
                groups text,
                source text not null,
                version text not null,
                epoch integer not null,
                installed_files text,
                provides text,
                conflicts text
            )",
        [],
    ).expect("Failed to insert installed packages table");

    conn.execute(
        "create table if not exists repos
            (
                name text not null unique primary key,
                repo_hash text not null,
                last_updated text not null
            )",
        [],
    ).expect("Failed to insert repos table");

    add_package_to_installed(NewPackage {
        name: "bulge".to_string(),
        groups: "core".to_string(),
        version: crate::get_version().to_string(),
        epoch: 0,
        installed_files: vec![],
        provides: vec!["bulge".to_string()],
        conflicts: vec![]
    }, Source{
        name: "core".to_string(),
        url: None
    });
}

/// Adds a package to the installed packages database
pub fn add_package_to_installed(package: NewPackage, source: Source) {
    let conn = Connection::open(get_root() + "/etc/bulge/databases/bulge.db").expect("Failed to create package database");

    // Convert installed files into a string
    let installed_files: String = vec_to_string(package.installed_files);

    // Convert source into a string
    let package_source: String;
    if source.url.is_none() {
        package_source = source.name
    } else {
        package_source = format!("{},{}", source.name, source.url.unwrap());
    }

    conn.execute("
        INSERT OR REPLACE INTO installed_packages (name, groups, source, version, epoch, installed_files, provides, conflicts)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
        params![package.name,
        package.groups,
        package_source,
        package.version,
        package.epoch,
        installed_files,
        vec_to_string(package.provides),
        vec_to_string(package.conflicts)]
    ).expect("Failed to insert package into database!");
}

/// Returns files owned by a package
pub fn return_owned_files(package: &String) -> Result<Vec<String>, rusqlite::Error> {
    let conn = Connection::open(get_root() + "/etc/bulge/databases/bulge.db")?;
    let mut files: Vec<String> = vec![];

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE name = ?")?;

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPackages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            epoch: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap())
        });
    })?;

    for pkg in result {
        files = pkg?.installed_files.clone();
    }

    Ok(files)
}

/// Removes a package from the installed packages database
pub fn remove_package_from_installed(package: &String) -> Result<(), rusqlite::Error>{
    let conn = Connection::open(get_root() + "/etc/bulge/databases/bulge.db")?;

    conn.execute("DELETE FROM installed_packages WHERE name = ?1",
    params![package])?;

    Ok(())
}

/// Look for a package in a repo and return the repo it is present in
pub fn search_for_package(package: &String) -> Result<String, PackageDBError> {
    let mut repo = String::new();

    for i in get_sources() {
        let conn = Connection::open(format!("{}/etc/bulge/databases/cache/{}.db", get_root(), i.name)).expect("Failed to create package database");

        // Fail silently and skip, this happens when the repo is empty
        if conn.prepare("SELECT * FROM packages WHERE name = ?").is_err() {
            println!("WARN> Repo {} is empty", i.name);
            continue;
        }

        let mut statement = conn.prepare("SELECT * FROM packages WHERE name = ?").expect("Failed to prepare statement");
        let mut rows = statement.query([package]).expect("Failed to query database");

        while let Some(_) = rows.next().expect("Failed to get next row") {
            repo = i.name.clone();
        }

        if !repo.is_empty() {
            return Ok(repo)
        }
    }

    return Ok(repo)
}

pub fn update_cached_repos(repo: &String, repo_hash: &String) {
    let conn = Connection::open(get_root() + "/etc/bulge/databases/bulge.db").expect("Failed to create package database");

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards?")
        .as_millis()
        .to_string();

    conn.execute("
        INSERT OR REPLACE INTO repos (name, repo_hash, last_updated)
        VALUES (?1, ?2, ?3);",
                 params![repo,
                 repo_hash,
                 current_time]
    ).expect("Failed to insert repo into database!");
}

// TODO: Change this to provides?
pub fn get_installed_package(package: &String) -> Result<InstalledPackages, PackageDBError> {
    let conn = Connection::open(get_root() + "/etc/bulge/databases/bulge.db").expect("Failed to open database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE name = ?").expect("Failed to prepare statement");

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPackages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            epoch: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap())
        });
    }).expect("DB Error!");

    for pkg in result {
        return Ok(pkg.unwrap());
    }

    return Err(PackageDBError);
}

pub fn get_remote_package(package: &String, repo: &String) -> Result<Package, PackageDBError> {
    let conn = Connection::open(format!("{}/etc/bulge/databases/cache/{}.db", get_root(), repo)).expect("Failed to open database");

    let statement = conn.prepare("SELECT * FROM packages WHERE name = ?");

    if statement.is_err() {
        return Err(PackageDBError);
    }

    let mut unwrap_statement = statement.unwrap();

    let result = unwrap_statement.query_map([package], | package | {
        return Ok(Package{
            name: package.get(0).unwrap(),
            version: package.get(1).unwrap(),
            epoch: package.get(2).unwrap(),
            description: package.get(3).unwrap(),
            groups: package.get(4).unwrap(),
            url: package.get(5).unwrap(),
            license: package.get(6).unwrap(),
            depends: package.get(7).unwrap(),
            optional_depends: package.get(8).unwrap(),
            provides: package.get(9).unwrap(),
            conflicts: package.get(10).unwrap(),
            replaces: package.get(11).unwrap(),
            sha512sum: package.get(12).unwrap()
        });
    }).expect("DB Error!");

    for pkg in result {
        return Ok(pkg.unwrap());
    }

    return Err(PackageDBError);
}


/// Get top-level dependencies for a package
pub fn get_dependencies(package_name: String) -> Result<Vec<Package>, PackageDBError> {
    let mut dependencies: Vec<Package> = Vec::new();

    let pkg_repo = search_for_package(&package_name)?;

    let mut pkg = get_remote_package(&package_name, &pkg_repo)?;

    if pkg.depends.is_empty() {
        return Ok(dependencies);
    }

    for dep in pkg.depends.split(",") {
        let mut dep_pkg = get_remote_package(&dep.to_string(), &pkg_repo)?;
        dependencies.push(dep_pkg);
    }

    return Ok(dependencies);
}

pub fn get_all_installed() -> Vec<InstalledPackages> {
    let conn = Connection::open(format!("{}/etc/bulge/databases/bulge.db", get_root())).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages").expect("Failed to create statement");

    let result = statement.query_map([], | package | {
        return Ok(InstalledPackages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            epoch: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap())
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

/// Look for a group in a repo and return the repo it is present in
pub fn search_for_group(group: &String) -> Result<String, PackageDBError> {
    let mut repo = String::new();

    for i in get_sources() {
        let conn = Connection::open(format!("{}/etc/bulge/databases/cache/{}.db", get_root(), i.name)).expect("Failed to create package database");

        // Fail silently and skip, this happens when the repo is empty
        if conn.prepare("SELECT * FROM packages WHERE instr(groups, ?) > 0;").is_err() {
            println!("WARN> Repo {} is empty", i.name);
            continue;
        }

        let mut statement = conn.prepare("SELECT * FROM packages WHERE instr(groups, ?) > 0;").expect("Failed to prepare statement");
        let mut rows = statement.query([group]).expect("Failed to query database");

        while let Some(_) = rows.next().expect("Failed to get next row") {
            repo = i.name.clone();
        }

        if !repo.is_empty() {
            return Ok(repo)
        }
    }

    return Ok(repo)
}

/// Get all packages in a requested group
pub fn get_group(repo: &String, group: &String) -> Vec<Package> {
    let conn = Connection::open(format!("{}/etc/bulge/databases/cache/{}.db", get_root(), repo)).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM packages WHERE instr(groups, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([group], | package | {
        return Ok(Package{
            name: package.get(0).unwrap(),
            version: package.get(1).unwrap(),
            epoch: package.get(2).unwrap(),
            description: package.get(3).unwrap(),
            groups: package.get(4).unwrap(),
            url: package.get(5).unwrap(),
            license: package.get(6).unwrap(),
            depends: package.get(7).unwrap(),
            optional_depends: package.get(8).unwrap(),
            provides: package.get(9).unwrap(),
            conflicts: package.get(10).unwrap(),
            replaces: package.get(11).unwrap(),
            sha512sum: package.get(12).unwrap()
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

pub fn get_provides(repo: &String, package: &String) -> Vec<Package> {
    let conn = Connection::open(format!("{}/etc/bulge/databases/cache/{}.db", get_root(), repo)).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM packages WHERE instr(provides, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([package], | package | {
        return Ok(Package{
            name: package.get(0).unwrap(),
            version: package.get(1).unwrap(),
            epoch: package.get(2).unwrap(),
            description: package.get(3).unwrap(),
            groups: package.get(4).unwrap(),
            url: package.get(5).unwrap(),
            license: package.get(6).unwrap(),
            depends: package.get(7).unwrap(),
            optional_depends: package.get(8).unwrap(),
            provides: package.get(9).unwrap(),
            conflicts: package.get(10).unwrap(),
            replaces: package.get(11).unwrap(),
            sha512sum: package.get(12).unwrap()
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

pub fn get_conflicts(package: &String) -> Vec<InstalledPackages> {
    let conn = Connection::open(format!("{}/etc/bulge/databases/bulge.db", get_root())).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE instr(conflicts, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPackages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            epoch: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap())
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}