use kdl::{KdlValue, KdlError, KdlNode};
use crate::get_xdg_direct;
use std::fs::File;
use std::io::prelude::*;

/// Returns a vec of the specified config entry
pub fn get_config_entry(entry: String) -> Result<Vec<KdlValue>, KdlError> {
    // Split input into vec for searching
    let mut path: Vec<&str> = entry.split(".").collect();

    // Load config file
    let mut x = String::new();
    
    File::open(get_xdg_direct().find_config_file("config.kdl").expect("Cannot find config, is one present?"))
        .expect("Failed to open config file, is another process accessing it?")
        .read_to_string(&mut x);

    // Load config nodes
    let config_nodes: Vec<KdlNode> = kdl::parse_document(x)?;

    // Search for entry
    let mut vec_object: Vec<KdlNode> = config_nodes.clone();

    while path.len() > 1 {
        let mut pos: usize = 0;
        for i in vec_object.clone().iter() {
            if i.name == path[0].to_string() {
                path.remove(0);
                vec_object = i.children.clone();
                break
            } else if (vec_object.len() -1) == pos {
                // If we're in the last position of the vec and it has not matched yet, assume that the requested config entry doesn't exist
                return Ok(vec![]) // TODO: Return an error here instead of just Oking it
            }
            pos += 1;
        }
    }

    // Return requested values
    Ok(vec_object[0].values.clone())
}