use kdl::{KdlValue, KdlNode};
use std::fs::File;
use std::io::prelude::*;
use crate::util::database::Source;
use std::fmt;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    config: ConfigNode,
    repos: Vec<RepoNode>
}

#[derive(Deserialize)]
struct ConfigNode {
    architecture: String,
    colour: bool,
    progressbar: bool
}

#[derive(Deserialize)]
struct RepoNode {
    name: String,
    active: bool,
    url: Option<String> 
}

pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error getting config!")
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

/// Returns a request KdlNode object, for use in other config entry getters
pub fn get_config_node(entry: &str) -> Result<Vec<KdlNode>, ConfigError> {
    // Split input into vec for searching
    let mut path: Vec<&str> = entry.split(".").collect();

    // Load config file
    let mut x = String::new();
    
    File::open("/etc/bulge/config.json")
        .expect("Failed to open config file, is another process accessing it?")
        .read_to_string(&mut x)
        .expect("Failed to convert file to string");

    // Load config nodes
    let config_nodes: Vec<KdlNode> = kdl::parse_document(x).expect("Cannot get config nodes");

    // Search for entry
    let mut vec_object: Vec<KdlNode> = config_nodes.clone();

    if path.len() == 1 { // FIXME: this is a horrible hack and creates unnecessary duplicated code
        let mut pos: usize = 0;
        for i in vec_object.clone().iter() {
            if i.name == path[0].to_string() {
                path.remove(0);
                vec_object = i.children.clone();
                break
            } else if (vec_object.len() -1) == pos{
                // If we're in the last position of the vec and it has not matched yet, assume that the requested config entry doesn't exist
                return Err(ConfigError{})
            }
            pos += 1;
        }

        // Return requested node early
        return Ok(vec_object.clone());
    }

    while path.len() > 1 {
        let mut pos: usize = 0;
        for i in vec_object.clone().iter() {
            if i.name == path[0].to_string() {
                path.remove(0);
                vec_object = i.children.clone();
                break
            } else if (vec_object.len() -1) == pos{
                // If we're in the last position of the vec and it has not matched yet, assume that the requested config entry doesn't exist
                return Err(ConfigError{})
            }
            pos += 1;
        }
    }

    // Return requested node
    Ok(vec_object.clone())
}

pub fn get_config_values(entry: &str) -> Vec<KdlValue> {
    let node = get_config_node(entry).expect("Failed to get node from config");

    node[0].values.clone()
}

pub fn get_config_children(entry: &str) -> Vec<KdlNode> {
    let node = get_config_node(entry).expect("Failed to get node from config");

    node.clone()
}

/// Return sources in config
pub fn get_sources() -> Vec<Source> {
    let mut sources: Vec<Source> = vec![];

    let repo_config_entry: Vec<KdlNode> = get_config_children("repos");

    for i in repo_config_entry {
        if i.properties.get("active").unwrap().to_string() == "true" {
            sources.push(Source{
                name: i.name,
                url: if i.properties.contains_key("url") { Option::from(i.properties.get("url").unwrap().to_string()) } else { None }
            })
        }
    }

    sources
}
