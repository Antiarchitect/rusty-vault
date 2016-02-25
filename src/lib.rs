mod crypt;

extern crate uuid;
use uuid::Uuid;

extern crate rustc_serialize;

use std::sync::mpsc::channel;
use std::thread;

mod storages;
use storages::{StorableKey, StorableData, StorableMap};
use storages::filesystem as Storage;

const KEYS_PATH: &'static str = "/home/andrey/Documents/storages/keys";
const DATA_PATH: &'static str = "/home/andrey/Documents/storages/data";
const MAPS_PATH: &'static str = "/home/andrey/Documents/storages/maps";

pub fn dump(external_id: String, data: Vec<u8>) -> Result<(), String>  {
    let result = crypt::encrypt(external_id.as_bytes(), &data);

    let (key_tx, key_rx) = channel::<Result<(), String>>();
    let storable = StorableKey { key: result.key, iv: result.iv };
    let storage = Storage::FilesystemStorage::new(KEYS_PATH);
    let key_id = Uuid::new_v4();
    thread::spawn(move || {
        key_tx.send(storage.dump(key_id.to_string(), storable).unwrap()).unwrap();
    });

    let (data_tx, data_rx) = channel::<Result<(), String>>();
    let storable = StorableData { ciphertext: result.ciphertext };
    let storage = Storage::FilesystemStorage::new(DATA_PATH);
    let data_id = Uuid::new_v4();
    thread::spawn(move || {
        data_tx.send(storage.dump(data_id.to_string(), storable).unwrap()).unwrap();
    });

    let (map_tx, map_rx) = channel::<Result<(), String>>();
    let storable = StorableMap { key_id: key_id, data_id: data_id, tag: result.tag };
    let storage = Storage::FilesystemStorage::new(MAPS_PATH);
    thread::spawn(move || {
        map_tx.send(storage.dump(external_id, storable).unwrap()).unwrap();
    });
}

pub fn load(external_id: &String) -> Vec<u8> {
    let storage = Storage::FilesystemStorage::new(MAPS_PATH);
    let map: storages::StorableMap = storage.load(external_id).ok().expect(format!("Nothing stored in maps for {}", external_id));

    let (key_tx, key_rx) = channel();
    let id = map.key_id.to_string();
    let storage = Storage::FilesystemStorage::new(KEYS_PATH);
    thread::spawn(move || {
        key_tx.send(storage.load(id).ok().expect(format!("Nothing stored in keys for {}", id)));
    });

    let (data_tx, data_rx) = channel();
    let id = map.data_id.to_string();
    let storage = Storage::FilesystemStorage::new(MAPS_PATH);
    thread::spawn(move || {
        data_tx.send(storage.load(id).ok().expect(format!("Nothing stored in data for {}", id)));
    });

    let key = key_rx.recv().unwrap();
    let data = data_rx.recv().unwrap();

    let result = crypt::decrypt(external_id.as_bytes(), &key.key, &key.iv, &data.ciphertext, &map.tag);

    result.plaintext
}
