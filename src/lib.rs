mod crypt;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate uuid;
use uuid::Uuid;

use std::io::prelude::*;
use std::fs;
use std::path;

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

fn store_json_string(path_prefix: String, path_key: Uuid, json: String) -> Result<(), String> {
    let store_path = construct_store_path(&path_prefix, &path_key.to_simple_string());
    fs::create_dir_all(&store_path).ok();
    let mut file = fs::File::create(construct_storable_path(&store_path, &path_key.to_string())).ok().expect("Cannot create file");
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

fn read_from_storage(prefix: &String, id: &String) -> Result<String, String> {
    let path = construct_storable_path(&construct_store_path(prefix, id), id);
    let mut file = fs::File::open(&path).unwrap();
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(error) => Err(format!("Error: {}", error))
    }
}

pub fn load(external_id: &String) -> Vec<u8> {
    let map: StorableMap = json::decode(&read_from_storage(&MAPS_PATH.to_string(), external_id).unwrap()).unwrap();
    let key: StorableKey = json::decode(&read_from_storage(&KEYS_PATH.to_string(), &map.key_id.to_string()).unwrap()).unwrap();
    let data: StorableData = json::decode(&read_from_storage(&DATA_PATH.to_string(), &map.data_id.to_string()).unwrap()).unwrap();

    let result = crypt::decrypt(external_id.as_bytes(), &key.key, &key.iv, &data.ciphertext, &map.tag);

    result.plaintext
}
