use std::fs::File;
use std::io::prelude::*;
use crate::get_xdg_direct;
use crate::util::conf::get_config_entry;

/// Load mirrors for repos from mirror list
pub fn load_mirrors() -> Vec<String> {
    let mut mirrors: Vec<String> = vec![];

    let mut arch = get_config_entry("config.architecture")
        .expect("Failed to get config entry!")[0]
        .to_string();

    let mut raw_mirrors = String::new();

    File::open(get_xdg_direct().find_config_file("mirrors").expect("Cannot find mirror list, is one present?"))
        .expect("Failed to open mirror list, is another program using it?")
        .read_to_string(&mut raw_mirrors);

    for i in raw_mirrors.lines() {
        if !i.is_empty() && !i.starts_with("#") {
            mirrors.push(
                i.to_string()
                    .trim()
                    .replace("$arch", arch.trim_matches(|c| c == '\\' || c == '"'))
            );
        }
    }

    return mirrors;
}