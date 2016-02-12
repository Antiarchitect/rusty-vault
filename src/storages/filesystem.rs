use std::io::prelude::*;
use std::fs;
use std::path;

use uuid::Uuid;

fn construct_store_path(path_prefix: &String, path_key_string: &String) -> path::PathBuf {
    let mut path = path::PathBuf::from(path_prefix);
    path.push(path_key_string[0..2].to_string());
    path.push(path_key_string[2..4].to_string());
    path.push(path_key_string[4..6].to_string());
    path
}

fn construct_storable_path(path_prefix: &path::PathBuf, path_key: &String) -> path::PathBuf {
    let mut path = path::PathBuf::from(path_prefix);
    path.push(path_key);
    path.set_extension("json");
    path
}

pub fn dump_json_string(path_prefix: String, path_key: Uuid, json: String) -> Result<(), String> {
    let store_path = construct_store_path(&path_prefix, &path_key.to_simple_string());
    fs::create_dir_all(&store_path).ok();
    let mut file = fs::File::create(construct_storable_path(&store_path, &path_key.to_string())).ok().expect("Cannot create file");
    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Error: {}", error))
    }
}

pub fn load_json_string(prefix: &String, id: &String) -> Result<String, String> {
    let path = construct_storable_path(&construct_store_path(prefix, id), id);
    let mut file = fs::File::open(&path).unwrap();
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(error) => Err(format!("Error: {}", error))
    }
}
