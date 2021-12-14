use rusqlite::Connection;
use crate::util::database::fns::get_all_installed;

use crate::util::database::structs::InstalledPackages;
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::string_to_vec;
use crate::util::packaging::structs::{NewPackage, Package};

pub fn list() {
    let result = get_all_installed();

    for i in result {
        let source = i.clone().source;
        let source_name = source.split(",").collect::<Vec<&str>>()[0];

        println!("{} {}-{} {}", i.name, i.version, i.epoch, source_name);
    }
}