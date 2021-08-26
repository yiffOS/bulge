use rusqlite::{params, Connection};
use crate::util::macros::vec_to_string;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Package {
    pub name: String,
    pub version: String,
    pub epoch: i32,
    pub description: String,
    pub groups: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub build_depends: Vec<String>,
    pub optional_depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub checksum: String
}

struct InstalledPackages {
    name: String,
    groups: Vec<String>,
    source: String,
    version: f32,
    epoch: i32
}

pub struct Source {
    pub name: String,
    pub url: Option<String>
}

/// Creates a database containing locally installed packages and various information
pub fn init_database() {
    let conn = Connection::open("/etc/bulge/databases/bulge.db").expect("Failed to create package database");

    conn.execute(
        "create table if not exists installed_packages
            (
                name text not null unique primary key,
                groups text,
                source text not null,
                version text not null,
                epoch integer not null
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

    add_package_to_installed(Package{
        name: "bulge".to_string(),
        version: crate::get_version().to_string(),
        epoch: 0,
        description: "An experimental package manager for yiffOS.".to_string(),
        groups: vec!["core".to_string()],
        url: "https://www.yiffos.ga/".to_string(),
        license: vec!["MIT".to_string()],
        depends: vec![],
        build_depends: vec!["rust".to_string(), "cargo".to_string()],
        optional_depends: vec![],
        provides: vec!["bulge".to_string()],
        conflicts: vec![],
        replaces: vec![],
        checksum: "".to_string()
    }, Source{
        name: "core".to_string(),
        url: None
    });
}

/// Adds a package to the installed packages database
pub fn add_package_to_installed(package: Package, source: Source) {
    let conn = Connection::open("/etc/bulge/databases/bulge.db").expect("Failed to create package database");

    // Convert groups into a string
    let package_groups: String = vec_to_string(package.groups);

    // Convert source into a string
    let package_source: String;
    if source.url.is_none() {
        package_source = source.name
    } else {
        package_source = format!("{},{}", source.name, source.url.unwrap());
    }

    conn.execute("
        INSERT OR REPLACE INTO installed_packages (name, groups, source, version, epoch)
        VALUES (?1, ?2, ?3, ?4, ?5);",
        params![package.name,
        package_groups,
        package_source,
        package.version,
        package.epoch]
    ).expect("Failed to insert package into database!");
}

/// Look for a package in a repo and return the repo it is present in
pub fn search_for_package(package: &String) -> String {
    String::new()
}

pub fn update_cached_repos(repo: &String, repo_hash: &String) {
    let conn = Connection::open("/etc/bulge/databases/bulge.db").expect("Failed to create package database");

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