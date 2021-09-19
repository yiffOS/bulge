use std::fmt::{self};
use serde::Deserialize;

/// Custom error type for config related errors.
pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Requested config entry not found!")
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

/// All possible config entries.
pub enum ConfigEntries {
    Architecture,
    Colour,
    Progressbar,
    Repos
}

/// All possible repo config entries.
pub enum RepoEntries {
    Name,
    Active,
    Url
}

/// Struct form of Bulge's config file.
#[derive(Deserialize)]
pub(super) struct Config {
    pub(super) architecture: String,
    pub(super) colour: bool,
    pub(super) progressbar: bool,
    pub(super) repos: Vec<RepoNode>
}

/// Struct form of repo config.
#[derive(Deserialize)]
pub(super) struct RepoNode {
    pub(super) name: String,
    pub(super) active: bool,
    pub(super) url: Option<String> 
}