use rusqlite::{params, Connection};
use std::fs;

struct Package {
    name: String,
    version: i32,
    epoch: i32,
    description: String,
    groups: Vec<String>,
    url: String,
    license: Vec<String>,
    depends: Vec<String>,
    build_depends: Vec<String>,
    optional_depends: Vec<String>,
    provides: Vec<String>,
    conflicts: Vec<String>,
    replaces: Vec<String>,
    checksum: String
}

struct InstalledPackages {
    name: String,
    groups: Vec<String>,
    source: String,
    version: i32,
    epoch: i32
}

struct Source {
    name: String,
    url: String
}

/// Creates a database containing locally installed packages and various information
pub fn init_database() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    fs::create_dir_all("/usr/local/share/bulge/");
    let conn = Connection::open("/usr/local/share/bulge/bulge.db").expect("Failed to create package database");

    conn.execute(
        "create table if not exists installed_packages
            (
                name text not null unique primary key,
                groups text[],
                source text not null,
                version integer not null,
                epoch integer not null
            )",
        [],
    ).expect("Failed to insert installed packages table");

    conn.execute(
        "create table if not exists repos
            (
                name text not null unique primary key,
                signing_key text not null,
                last_updated text not null
            )",
        [],
    ).expect("Failed to insert repos table");

    println!("bulge: Created local database")
}