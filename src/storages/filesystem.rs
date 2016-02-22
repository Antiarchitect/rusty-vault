use std::io::prelude::*;
use std::fs;
use std::path;

use uuid::Uuid;

use rustc_serialize::json;

const KEYS_PATH: &'static str = "/home/andrey/Documents/storages/keys";
const DATA_PATH: &'static str = "/home/andrey/Documents/storages/data";
const MAPS_PATH: &'static str = "/home/andrey/Documents/storages/maps";

#[derive(RustcDecodable, RustcEncodable)]
pub struct StorableKey {
    pub key: Box<[u8]>,
    pub iv: Box<[u8]>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct StorableData {
    pub ciphertext: Vec<u8>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct StorableMap {
    pub key_id: Uuid,
    pub data_id: Uuid,
    pub tag: Box<[u8]>
}


pub fn store_data(storable: StorableData) -> Result<Uuid, String> {
    let id = Uuid::new_v4();
    let json = json::encode(&storable).unwrap();
    dump_json_string(DATA_PATH.to_string(), id, json).ok();
    Ok(id)
}

pub fn store_key(storable: StorableKey) -> Result<Uuid, String> {
    let id = Uuid::new_v4();
    let json = json::encode(&storable).unwrap();
    dump_json_string(KEYS_PATH.to_string(), id, json).ok();
    Ok(id)
}

pub fn store_map(external_id: Uuid, storable: StorableMap) -> Result<(), String> {
    let id = external_id;
    let json = json::encode(&storable).unwrap();
    dump_json_string(MAPS_PATH.to_string(), id, json).ok();
    Ok(())
}

pub fn load_data(data_id: &String) -> Result<StorableData, String> {
    match json::decode(&load_json_string(&DATA_PATH.to_string(), &data_id).unwrap()) {
        Ok(value) => Ok(value),
        Err(error) => Err(format!("Error: {}", error))
    }
}

pub fn load_key(key_id: &String) -> Result<StorableKey, String> {
    match json::decode(&load_json_string(&KEYS_PATH.to_string(), &key_id).unwrap()) {
        Ok(value) => Ok(value),
        Err(error) => Err(format!("Error: {}", error))
    }
}

pub fn load_map(external_id: &String) -> Result<StorableMap, String> {
    match json::decode(&load_json_string(&MAPS_PATH.to_string(), &external_id).unwrap()) {
        Ok(value) => Ok(value),
        Err(error) => Err(format!("Error: {}", error))
    }
}


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

fn dump_json_string(path_prefix: String, path_key: Uuid, json: String) -> Result<(), String> {
    let store_path = construct_store_path(&path_prefix, &path_key.to_simple_string());
    fs::create_dir_all(&store_path).ok();
    let mut file = fs::File::create(construct_storable_path(&store_path, &path_key.to_string())).ok().expect("Cannot create file");
    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Error: {}", error))
    }
}

fn load_json_string(prefix: &String, id: &String) -> Result<String, String> {
    let path = construct_storable_path(&construct_store_path(prefix, id), id);
    let mut file = fs::File::open(&path).unwrap();
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(error) => Err(format!("Error: {}", error))
    }
}
