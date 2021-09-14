use std::fs::File;
use std::io::prelude::*;
use crate::util::conf::{get_config_entry, ConfigEntries};

/// Load mirrors for repos from mirror list
pub fn load_mirrors() -> Vec<String> {
    let mut mirrors: Vec<String> = vec![];

    let arch = get_config_entry(ConfigEntries::Architecture, None, None).expect("Failed to get config architecture.");

    let mut raw_mirrors = String::new();

    File::open("/etc/bulge/mirrors")
        .expect("Failed to open mirror list, is another program using it?")
        .read_to_string(&mut raw_mirrors)
        .expect("Failed to convert file to string");

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