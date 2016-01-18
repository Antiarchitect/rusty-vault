mod crypt;

use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate uuid;
use uuid::Uuid;

use std::io::prelude::*;
use std::fs::File;

#[derive(RustcDecodable, RustcEncodable)]
struct StorableKey {
    key: Box<[u8]>,
    iv: Box<[u8]>
}

#[derive(RustcDecodable, RustcEncodable)]
struct StorableData {
    ciphertext: Box<Vec<u8>>
}

#[derive(RustcDecodable, RustcEncodable)]
struct StorableMap {
    external_id: Uuid,
    key_id: Uuid,
    data_id: Uuid
}

pub fn dump(external_id: String, data: Vec<u8>) -> Result<(), &'static str>  {

    let data_path = "/home/andrey/Documents/storages/data".to_string();
    let keys_path = "/home/andrey/Documents/storages/keys".to_string();
    let maps_path = "/home/andrey/Documents/storages/maps".to_string();

    let result = crypt::encrypt(external_id.as_bytes(), &data);

    let key_id = store_key(keys_path, StorableKey { key: result.key.key, iv: result.key.iv }).unwrap();
    let data_id = store_data(data_path, StorableData { ciphertext: result.ciphertext }).unwrap();
    store_map(maps_path, StorableMap { external_id: Uuid::parse_str(&external_id).unwrap(), key_id: key_id, data_id: data_id})
}

fn store_data(path_prefix: String, storable: StorableData) -> Result<Uuid, &'static str> {
    let id = Uuid::new_v4();
    let encoded_storable = json::encode(&storable).unwrap();

    let mut file = File::create(Path::new(&format!("{}/{}.json", path_prefix, id.to_string()))).ok().expect("Cannot create file");
    match file.write_all(encoded_storable.as_bytes()) {
        Ok(_) => Ok(id),
        Err(error) => Err("fucked")
    }
}

fn store_key(path_prefix: String, storable: StorableKey) -> Result<Uuid, &'static str> {
    let id = Uuid::new_v4();
    let encoded_storable = json::encode(&storable).unwrap();

    let mut file = File::create(Path::new(&format!("{}/{}.json", path_prefix, id.to_string()))).ok().expect("Cannot create file");
    match file.write_all(encoded_storable.as_bytes()) {
        Ok(_) => Ok(id),
        Err(error) => Err("fucked")
    }
}

fn store_map(path_prefix: String, storable: StorableMap) -> Result<(), &'static str> {
    let id = Uuid::new_v4();
    let encoded_storable = json::encode(&storable).unwrap();

    let mut file = File::create(Path::new(&format!("{}/{}.json", path_prefix, id.to_string()))).ok().expect("Cannot create file");
    match file.write_all(encoded_storable.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err("fucked")
    }
}

pub struct LoadResult {
    pub data: Vec<u8>
}

trait FakeLoadResult {
    fn new(data: &'static str) -> Self;
    fn data(&self) -> Vec<u8> { self.data() }
}

impl FakeLoadResult for LoadResult {
    fn new(data: &'static str) -> LoadResult {
        LoadResult { data: data.to_string().into_bytes() }
    }
}

pub fn load(external_id: &String) -> LoadResult {
    LoadResult::new("fakevalue")
}
