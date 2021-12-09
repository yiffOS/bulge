use std::fs::File;
use std::io::prelude::*;
use crate::util::database::structs::Source;
use crate::util::config::structs::{ConfigEntries, ConfigError, Config, RepoEntries, RepoNode};
use crate::util::macros::get_root;


/// Returns a string of the requested config entry, optionally returns a config entry within a repo.
///
/// See [ConfigEntries] and [RepoEntries].
pub fn get_config_entry(entry: ConfigEntries, repo: Option<String>, repo_entry: Option<RepoEntries>) -> Result<String, ConfigError> {
    // Load config file
    let mut x = String::new();
    
    File::open(get_root() + "/etc/bulge/config.json")
        .expect("Failed to open config file, is another process accessing it?")
        .read_to_string(&mut x)
        .expect("Failed to convert file to string");

    let config: Config = serde_json::from_str(&x).expect("Failed to serialize data");

    match entry {
        ConfigEntries::Architecture => Ok(config.architecture),
        ConfigEntries::Toolchain => Ok(config.toolchain),
        ConfigEntries::Colour => Ok(config.colour.to_string()),
        ConfigEntries::Progressbar => Ok(config.progressbar.to_string()),
        ConfigEntries::Repos => {
            // Check if a repo and a repo config entry were supplied
            if repo.is_none() && repo_entry.is_none() {
                for i in config.repos {
                    // Find the requested repo
                    if repo.clone().unwrap() == i.name {
                        // Return the requested repo config entry
                        match repo_entry.unwrap() {
                            RepoEntries::Name => return Ok(i.name),
                            RepoEntries::Active => return Ok(i.active.to_string()),
                            RepoEntries::Url => {
                                if i.url.is_some() {
                                    return Ok(i.url.unwrap());
                                }
                                // If a url is not present, return an empty string
                                return Ok(String::new());
                            },
                        }
                    }
                }
            }
            return Err(ConfigError)
        },
    }
}

/// Returns a Vec containing the entire repos array from config.
///
/// Currently only used for [get_sources].
fn get_repo_vec() -> Vec<RepoNode> {
        // Load config file
        let mut x = String::new();
    
        File::open(get_root() + "/etc/bulge/config.json")
            .expect("Failed to open config file, is another process accessing it?")
            .read_to_string(&mut x)
            .expect("Failed to convert file to string");
    
        let config: Config = serde_json::from_str(&x).expect("Failed to serialize data");

        return config.repos
}

/// Return sources in config.
///
/// See [Source].
pub fn get_sources() -> Vec<Source> {
    let mut sources: Vec<Source> = vec![];

    let repo_config_entry: Vec<RepoNode> = get_repo_vec();

    for i in repo_config_entry {
        if i.active == true {
            sources.push(Source{
                name: i.name,
                url: if i.url.is_some() { Option::from(i.url.unwrap()) } else { None }
            })
        }
    }

    return sources;
}
