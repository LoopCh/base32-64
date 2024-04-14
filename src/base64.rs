use std::collections::HashMap;
use std::fs::File;
use serde_json;

use crate::utils;
pub struct Base64;

impl Base64 {
    pub fn encode(data: &String) -> String {
        let chunk_size = 6;
        let file = File::open("src/base64.json").unwrap();
        let config: HashMap<String, String> = serde_json::from_reader(file).unwrap();

        let bytes = data.as_bytes()
            .iter()
            .map(|el| format!("{:08b}", el))
            .collect::<Vec<String>>()
            .join("");


        let split_bytes = utils::split_by_length(&bytes, chunk_size);

        let split_decimal = split_bytes.chunks(4)
            .map(|chunk|{
                let mut copy = chunk.to_vec();
                for el in &mut copy {
                    while el.len() != chunk_size {
                        el.push('0');
                    };
                };
                
                copy.iter()
                    .map(|el| u32::from_str_radix(&el, 2).expect("error").to_string())
                    .collect::<Vec<_>>()
                    
            }).collect::<Vec<_>>();
            
        let mut result = String::new();

        for el in split_decimal.iter() {

            for i in el {
                result.push_str(config[i].as_str());
            };
            
            for _ in el.len()..4 {
                result.push('=');
            };
        } 
        result
        
    }

    pub fn decode(data: &String) -> String {

        let data = data.replace("=", "");

        let file = File::open("src/base64.json").unwrap();
        let config: HashMap<String, String> = serde_json::from_reader(file).unwrap();

        let decimal = data
            .chars()
            .map(|el| 
                format!("{:06b}",utils::get_key_for_value(&config, &el.to_string()).unwrap()
                .parse::<u32>().unwrap())
            )
            .collect::<Vec<_>>()
            .join("");

        let mut split_decimal = utils::split_by_length(&decimal, 8);
        split_decimal.retain(|el| el.len() == 8);

        let mut result = Vec::new();

        for el in split_decimal {
            result.push(u8::from_str_radix(&el, 2).unwrap());
        }

        let result = String::from_utf8(result).unwrap();

        result
    }

}