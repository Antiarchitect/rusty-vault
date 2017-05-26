use std::io::prelude::*;
use std::fs;
use std::path;

use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};

use super::VaultStorage;
use super::StorageResult;
use super::StorageResultOption;

pub struct Storage {
    pub path: &'static str
}

impl Storage {

    fn ensure_storage_path(&self, key: &String) -> StorageResult<path::PathBuf> {
        let mut path = path::PathBuf::from(self.path);
        path.push(key);
        path.push(key[0..2].to_string());
        path.push(key[2..4].to_string());
        path.push(key[4..6].to_string());
        fs::create_dir_all(&path)?;
        path.push(key);
        path.set_extension("json");
        Ok(path)
    }

}

impl VaultStorage for Storage {

    fn dump<T: Encodable>(&self, id: &String, storable: T) -> StorageResult<()> {
        let path = self.ensure_storage_path(id)?;
        let mut storage = fs::File::create(&path)?;
        storage.write_all(json::encode(&storable).unwrap().as_bytes())?;
        Ok(())
    }

    fn delete(&self, id: &String) -> StorageResultOption<()> {
        let path = self.ensure_storage_path(id)?;
        if !(path.exists()) { return Ok(None) };
        fs::remove_file(path)?;
        Ok(Some(()))
    }

    fn load<T: Decodable>(&self, id: &String) -> StorageResultOption<T> {
        let path = self.ensure_storage_path(id)?;
        if !(path.exists()) { return Ok(None) };
        let mut storage = fs::File::open(&path)?;
        let mut string = String::new();
        storage.read_to_string(&mut string)?;
        Ok(Some(json::decode(&string)?))
    }
}