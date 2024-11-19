use std::{collections::{BTreeMap, HashMap}, fs::File, io::{Read, Write}};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Run {
    pub step_count: u128
}

// Key Value Pair: <spin sector: <chain_length: Vec<Run>>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunData {
    pub runs: BTreeMap<usize, Vec<u128>>
}

impl RunData {
    pub fn new() -> RunData {
        let mut runs: BTreeMap<usize, Vec<u128>> = BTreeMap::new();
        let run_data = RunData{runs};
        run_data
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZData{
    pub z_data: HashMap<usize, Vec<f64>>
}

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