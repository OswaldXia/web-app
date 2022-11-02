// To manage the read and write to the JSON file
use serde_json::{json, value::Value, Map};
use std::{
    fs::{self, File},
    io::Read,
};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = serde_json::from_str::<Value>(&data).unwrap();
    let state = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    // let mut file = File::open(file_name).unwrap();
    // let json = serde_json::to_string(&state).unwrap();
    // file.write(json.as_bytes());
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write file");
}
