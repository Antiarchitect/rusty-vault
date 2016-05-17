use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::path;

use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};

pub struct Storage {
    pub path: &'static str
}

impl super::MapsStorage for Storage {}
impl super::KeysStorage for Storage {}
impl super::DataStorage for Storage {}

impl Storage {

    fn ensure_storage_path(&self, key: &String) -> Result<path::PathBuf, Box<Error>> {
        let mut path = path::PathBuf::from(self.path);
        path.push(key);
        path.push(key[0..2].to_string());
        path.push(key[2..4].to_string());
        path.push(key[4..6].to_string());
        try!(fs::create_dir_all(&path));
        path.push(key);
        path.set_extension("json");
        Ok(path)
    }

}

impl super::BaseStorage for Storage {

    fn dump<T: Encodable>(&self, id: &String, storable: T) -> Result<(), Box<Error>> {
        let path = try!(self.ensure_storage_path(id));
        let mut storage = try!(fs::File::create(&path));
        try!(storage.write_all(json::encode(&storable).unwrap().as_bytes()));
        Ok(())
    }

    fn delete(&self, id: &String) -> Result<Option<()>, Box<Error>> {
        let path = try!(self.ensure_storage_path(id));
        if !(path.exists()) { return Ok(None) };
        try!(fs::remove_file(path));
        Ok(Some(()))
    }

    fn load<T: Decodable>(&self, id: &String) -> Result<Option<T>, Box<Error>> {
        let path = try!(self.ensure_storage_path(id));
        if !(path.exists()) { return Ok(None) };
        let mut storage = try!(fs::File::open(&path));
        let mut string = String::new();
        try!(storage.read_to_string(&mut string));
        Ok(Some(try!(json::decode(&string))))
    }
}
