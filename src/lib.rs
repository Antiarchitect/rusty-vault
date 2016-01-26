mod crypt;

use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate uuid;
use uuid::Uuid;

use std::io::prelude::*;
use std::fs;

const KEYS_PATH: &'static str = "/home/andrey/Documents/storages/keys";
const DATA_PATH: &'static str = "/home/andrey/Documents/storages/data";
const MAPS_PATH: &'static str = "/home/andrey/Documents/storages/maps";

#[derive(RustcDecodable, RustcEncodable)]
struct StorableKey {
    key: Box<[u8]>,
    iv: Box<[u8]>
}

#[derive(RustcDecodable, RustcEncodable)]
struct StorableData {
    ciphertext: Vec<u8>
}

#[derive(RustcDecodable, RustcEncodable)]
struct StorableMap {
    key_id: Uuid,
    data_id: Uuid,
    tag: Box<[u8]>
}

pub fn dump(external_id: String, data: Vec<u8>) -> Result<(), String>  {
    let result = crypt::encrypt(external_id.as_bytes(), &data);

    let key_id = store_key(KEYS_PATH.to_string(), StorableKey { key: result.key, iv: result.iv }).unwrap();
    let data_id = store_data(DATA_PATH.to_string(), StorableData { ciphertext: result.ciphertext }).unwrap();
    let external_uuid = Uuid::parse_str(&external_id).unwrap();
    store_map(MAPS_PATH.to_string(), external_uuid, StorableMap { key_id: key_id, data_id: data_id, tag: result.tag })
}

fn prepare_full_path(path_prefix: &String, path_key_string: &String) -> String {
    format!("{}/{}/{}/{}", path_prefix, path_key_string[0..2].to_string(), path_key_string[2..4].to_string(), path_key_string[4..6].to_string())
}

fn store_json_string(path_prefix: String, path_key: Uuid, json: String) -> Result<(), String> {
    let full_path = prepare_full_path(&path_prefix, &path_key.to_simple_string());
    fs::create_dir_all(&full_path).ok();
    let mut file = fs::File::create(Path::new(&format!("{}/{}.json", full_path, path_key.to_string()))).ok().expect("Cannot create file");
    match file.write_all(json.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Error: {}", error))
    }
}

fn store_data(path_prefix: String, storable: StorableData) -> Result<Uuid, String> {
    let id = Uuid::new_v4();
    let json = json::encode(&storable).unwrap();
    store_json_string(path_prefix, id, json).ok();
    Ok(id)
}

fn store_key(path_prefix: String, storable: StorableKey) -> Result<Uuid, String> {
    let id = Uuid::new_v4();
    let json = json::encode(&storable).unwrap();
    store_json_string(path_prefix, id, json).ok();
    Ok(id)
}

fn store_map(path_prefix: String, external_id: Uuid, storable: StorableMap) -> Result<(), String> {
    let id = external_id;
    let json = json::encode(&storable).unwrap();
    store_json_string(path_prefix, id, json).ok();
    Ok(())
}

pub fn load(external_id: &String) -> Vec<u8> {
    let maps_path = prepare_full_path(&MAPS_PATH.to_string(), external_id);
    vec![0]
}
