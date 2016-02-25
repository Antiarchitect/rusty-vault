use std::io::prelude::*;
use std::fs;
use std::path;

use rustc_serialize::json;
use rustc_serialize::serialize::{Decodable, Encodable};
use std::collections::HashMap;

use super::{StorableKey, StorableData, StorableMap};

struct Storage {
    path: String
}

trait FilesystemStorage {

    fn new(path: &'static str) -> Self;

    fn ensure_storage_path(&self, key: String) -> path::PathBuf;

    fn dump<T: Encodable>(&self, id: String, storable: T) -> Result<(), String> {
        let path = self.ensure_storage_path(id);
        let mut storage = fs::File::create(&path).ok().expect(&format!("Cannot create file {}", path.to_string_lossy()));
        match storage.write_all(json::encode(&storable).unwrap().as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Error: {}", error))
        }
    }

    fn load<T: Decodable>(&self, id: String) -> Result<T, String> {
        let path = self.ensure_storage_path(id);
        let mut storage = fs::File::open(&path).ok().expect(&format!("Cannot open file {}", path.to_string_lossy()));
        let mut string = String::new();
        storage.read_to_string(&mut string).ok().expect(&format!("File content is invalid {}", path.to_string_lossy()));
        match json::decode(&string) {
            Ok(value) => Ok(value),
            Err(error) => Err(format!("Error: {}", error))
        }
    }
}

impl FilesystemStorage for Storage {
    fn new(path: &'static str) -> Storage {
        Storage { path: path.to_string() }
    }

    fn ensure_storage_path(&self, key: String) -> path::PathBuf {
        let mut path = path::PathBuf::from(self.path);
        path.push(key[0..2].to_string());
        path.push(key[2..4].to_string());
        path.push(key[4..6].to_string());
        fs::create_dir_all(&path).ok().expect(&format!("Cannot create store path {}", path.to_string_lossy()));
        path.push(key);
        path.set_extension("json");
        path
    }
}
