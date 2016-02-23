use std::io::prelude::*;
use std::fs;
use std::path;

use uuid::Uuid;

use rustc_serialize::json;

const KEYS_PATH: &'static str = "/home/andrey/Documents/storages/keys";
const DATA_PATH: &'static str = "/home/andrey/Documents/storages/data";
const MAPS_PATH: &'static str = "/home/andrey/Documents/storages/maps";

struct Storage {
    path: String
}

trait FilesystemStorage {

    fn new(path: String) -> Self;

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

    fn dump_json(&self, path_key: &String, json: String) -> Result<(), String> {
        let store_path = self.construct_store_path(&self.path, path_key);
        fs::create_dir_all(&store_path).ok();
        let mut file = fs::File::create(self.construct_storable_path(&store_path, path_key)).ok().expect("Cannot create file");
        match file.write_all(json.as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Error: {}", error))
        }
    }
}

trait DataStorage: FilesystemStorage {
    fn dump(id: String, storable: StorableData) -> Result<(), String>;
    fn load(id: String) -> Result<StorableData, String>;
}

trait KeysStorage: FilesystemStorage {
    fn dump(id: String, storable: StorableKey) -> Result<(), String>;
    fn load(id: String) -> Result<StorableKey, String>;
}

trait MapsStorage: FilesystemStorage {
    fn dump(id: String, storable: StorableMap) -> Result<(), String>;
    fn load(id: String) -> Result<StorableMap, String>;
}

impl FilesystemStorage for Storage {
    fn new(path: String) -> Storage {
        Storage { path: path }
    }
}

impl DataStorage for FilesystemStorage {
    fn dump(&self, id: String, storable: StorableData) -> Result<(), String> {
        self.dump_json(&id, json::encode(&storable).unwrap())
    }
    fn load(id: String) -> Result<StorableData, String> {

    }
}

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
