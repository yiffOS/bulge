use rusqlite::{params, Connection};

struct Package {
    name: String,
    version: i32,
    epoch: i32,
    description: String,
    groups: Vec<String>,
    arch: Vec<String>,
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
    source: String,
    version: i32
}

struct Source {
    name: String,
    url: String
}

/// Creates a database containing locally installed packages
pub fn init_database() {
    let conn = Connection::open("/etc/bulge/packages.db").expect("Failed to create database");

    conn.execute(
        "create table if not exists installed_packages
            (
                name text not null unique primary key,
                groups text[],
                source text not null,
                version integer not null
            )",
        [],
    ).expect("Failed to insert installed packages table");
}