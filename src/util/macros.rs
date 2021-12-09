use std::env;
use isahc::http::Error;
use isahc::{Body, Request, Response};
use isahc::config::RedirectPolicy;
use isahc::prelude::*;

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