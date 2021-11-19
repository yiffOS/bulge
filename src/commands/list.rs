use rusqlite::Connection;
use crate::util::database::structs::InstalledPackages;
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::string_to_vec;
use crate::util::packaging::structs::{NewPackage, Package};

pub fn list() {
    let conn = Connection::open("/etc/bulge/databases/bulge.db").expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages").expect("Failed to create statement");

    let result = statement.query_map([], | package | {
        return Ok(InstalledPackages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            epoch: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
        });
    }).expect("Failed to execute query");

    for i in result {
        let package = i.unwrap();
        println!("{} {}-{} {}", package.name, package.version, package.epoch, package.source);
    }
}