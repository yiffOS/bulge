use std::collections::{HashMap, HashSet};
use std::{env, io};
use std::io::Write;
use isahc::http::Error;
use isahc::{Body, Request, Response};
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use crate::util::packaging::structs::Package;

/// Converts a vec of strings to a flat string separated by ","
pub fn vec_to_string(vec: Vec<String>) -> String {
    let mut temp_string: String = String::new();
    let mut x: usize = 0;
    for i in &vec {
        temp_string.push_str(&*i);
        if !(x == (&vec.len() - 1)) {
            temp_string.push_str(",");
        }
        x += 1;
    }
    temp_string
}

pub fn display_installing_packages(set: HashMap<Package, String>) -> String {
    let mut temp_string: String = String::new();
    for i in set {
        temp_string.push_str(&*i.0.name);
        temp_string.push_str("-");
        temp_string.push_str(&*i.0.version);
        temp_string.push_str("-");
        temp_string.push_str(&*i.0.epoch.to_string());
        temp_string.push_str(" ");
    }
    temp_string
}

/// Converts a string separated by "," to a vec of strings 
pub fn string_to_vec(vec: String) -> Vec<String> {
    vec.split(",").map(|s| s.to_string()).collect()
}

/// Gets the root from the INSTALL_ROOT env variable
pub fn get_root() -> String {
    match env::var("INSTALL_ROOT") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    }
}

/// Default isahc get
pub fn get(url: &String) -> Result<Response<Body>, isahc::Error> {
    return Request::get(url)
            .redirect_policy(RedirectPolicy::Follow)
            .body(())?
            .send();
}

pub fn continue_prompt() -> bool {
    let mut input = String::new();

    print!("Continue? [y/n]: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        return true;
    }

    return false;
}