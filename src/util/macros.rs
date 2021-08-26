/// Converts a vec of strings to a flat string separated by ","
pub fn vec_to_string(vec: Vec<String>) -> String {
    let mut temp_string: String = String::new();
    for i in vec {
        temp_string.push_str(&*i);
        temp_string.push_str(",");
    }
    temp_string
}