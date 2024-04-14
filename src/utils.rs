use std::collections::HashMap;

pub fn split_by_length(input: &String, chunk_size: usize) -> Vec<String> {
 input.chars()
    .collect::<Vec<char>>()
    .chunks(chunk_size)
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<String>>()
}

pub fn get_key_for_value(map: &HashMap<String, String>, value_to_find: &str) -> Option<String> {
    for (key, value) in map.iter() {
        if value == value_to_find {
            return Some(key.clone());
        }
    }
    None
}