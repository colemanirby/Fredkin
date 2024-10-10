use std::{fs::File, io::{Read, Write}};

use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub fn load_data<T: DeserializeOwned>(file_name: String) -> T {

    let mut file = File::open(file_name).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    file.read_to_end(&mut buff).unwrap();
    let data_object: T = serde_json::from_slice(&buff).unwrap();
    data_object

}

pub fn save_data<T: Serialize>(file_name: String, data: &T) {
    let mut file = File::create(file_name).unwrap();
    let data = serde_json::to_string(data).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}