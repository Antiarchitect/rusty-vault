use std::io::prelude::*;
use std::fs;
use std::path;

use uuid::Uuid;

use rustc_serialize::json;
use std::collections::HashMap;

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

    fn dump(&self, path_key: String, hash: HashMap) -> Result<(), String> {
        let store_path = self.construct_store_path(&self.path, path_key);
        fs::create_dir_all(&store_path).ok();
        let mut file = fs::File::create(self.construct_storable_path(&store_path, path_key)).ok().expect("Cannot create file");
        match file.write_all(json::encode(hash).unwrap().as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Error: {}", error))
        }
    }
}

impl FilesystemStorage for Storage {
    fn new(path: String) -> Storage {
        Storage { path: path }
    }
}

trait StorableHashMap {
    fn to_hash(&self) -> HashMap;
}

impl StorableHashMap for StorableKey {
    fn to_hash(&self) -> HashMap {
        let mut map = HashMap::new();
        map.insert("key", self.key);
        map.insert("iv", self.iv);
        map
    }
}

impl StorableHashMap for StorableData {
    fn to_hash(&self) -> HashMap {
        let mut map = HashMap::new();
        map.insert("ciphertext", self.ciphertext);
        map
    }
}

impl StorableHashMap for StorableMap {
    fn to_hash(&self) -> HashMap {
        let mut map = HashMap::new();
        map.insert("key_id", self.key_id);
        map.insert("data_id", self.data_id);
        map.insert("tag", self.tag);
        map
    }
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
