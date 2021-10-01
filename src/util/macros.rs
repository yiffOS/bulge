/// Converts a vec of strings to a flat string separated by ","
pub fn vec_to_string(vec: Vec<String>) -> String {
    let mut temp_string: String = String::new();
    for i in vec {
        temp_string.push_str(&*i);
        temp_string.push_str(",");
    }
    temp_string
}

/// Converts a string separated by "," to a vec of strings 
pub fn string_to_vec(vec: String) -> Vec<String> {
    vec.split(",").map(|s| s.to_string()).collect()
}