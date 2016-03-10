use std::sync::mpsc::channel;
use std::thread;
use std::error::Error;

mod crypt;

extern crate uuid;
use uuid::Uuid;

extern crate rustc_serialize;

mod storages;
use storages::{StorableKey, StorableData, StorableMap};
use storages::filesystem as storage;

const KEYS_PATH: &'static str = "/home/andrey/Documents/storages/keys";
const DATA_PATH: &'static str = "/home/andrey/Documents/storages/data";
const MAPS_PATH: &'static str = "/home/andrey/Documents/storages/maps";

pub fn dump(external_id: &String, data: Vec<u8>) -> Result<(), Box<Error>>  {

    try!(delete(external_id)); // To replace existing object we should remove the previous one.

    let encrypted = crypt::encrypt(external_id.as_bytes(), &data);
    let key_id = Uuid::new_v4();
    let data_id = Uuid::new_v4();

    let (tx, rx) = channel();

    let storable = StorableKey { key: encrypted.key, iv: encrypted.iv };
    let key_tx = tx.clone();
    thread::spawn(move || {
        let storage = storage::Storage::new(KEYS_PATH);
        key_tx.send(storage.dump(&key_id.to_string(), storable).map_err( |e| e.to_string() ))
    });

    let storable = StorableData { ciphertext: encrypted.ciphertext };
    let data_tx = tx.clone();
    thread::spawn(move || {
        let storage = storage::Storage::new(DATA_PATH);
        data_tx.send(storage.dump(&data_id.to_string(), storable).map_err( |e| e.to_string() ))
    });

    let storable = StorableMap { key_id: key_id, data_id: data_id, tag: encrypted.tag };
    let map_tx = tx.clone();
    let map_id = external_id.clone();
    thread::spawn(move || {
        let storage = storage::Storage::new(MAPS_PATH);
        map_tx.send(storage.dump(&map_id, storable).map_err( |e| e.to_string() ))
    });

    let results = (0..3).map(|_| rx.recv() ).collect::<Result<Vec<_>, _>>().unwrap();

    match results.into_iter().all( |i| i.is_ok() ) {
        true => Ok(()),
        false => Err(From::from("Cannot dump object."))
    }
}

pub fn load(external_id: &String) -> Result<Vec<u8>, Box<Error>> {
    let storage = storage::Storage::new(MAPS_PATH);
    let map: storages::StorableMap = try!(storage.load(external_id));

    let (key_tx, key_rx) = channel();
    let id = map.key_id.to_string();
    let storage = storage::Storage::new(KEYS_PATH);
    thread::spawn(move || {
        key_tx.send(storage.load(&id).map_err( |e| e.to_string() ))
    });

    let (data_tx, data_rx) = channel();
    let id = map.data_id.to_string();
    let storage = storage::Storage::new(DATA_PATH);
    thread::spawn(move || {
        data_tx.send(storage.load(&id).map_err( |e| e.to_string() ))
    });

    let key: StorableKey = try!(key_rx.recv().unwrap());
    let data: StorableData = try!(data_rx.recv().unwrap());

    let result = crypt::decrypt(external_id.as_bytes(), &key.key, &key.iv, &data.ciphertext, &map.tag);

    Ok(result.plaintext)
}

pub fn delete(external_id: &String) -> Result<Option<()>, Box<Error>> {
    let storage = storage::Storage::new(MAPS_PATH);

    let map: storages::StorableMap = match storage.load(external_id) {
        Ok(value) => value,
        Err(_) => return Ok(None)
    };

    let (tx, rx) = channel();

    let map_tx = tx.clone();
    let id = external_id.clone();
    thread::spawn(move || {
        map_tx.send(storage.delete(&id).map_err( |e| e.to_string() ))
    });

    let id = map.key_id.to_string();
    let storage = storage::Storage::new(KEYS_PATH);
    let key_tx = tx.clone();
    thread::spawn(move || {
        key_tx.send(storage.delete(&id).map_err( |e| e.to_string() ))
    });

    let id = map.data_id.to_string();
    let storage = storage::Storage::new(DATA_PATH);
    let data_tx = tx.clone();
    thread::spawn(move || {
        data_tx.send(storage.delete(&id).map_err( |e| e.to_string() ))
    });

    let results = (0..3).map(|_| rx.recv() ).collect::<Result<Vec<_>, _>>().unwrap();
    match results.into_iter().all( |i| i.is_ok() ) {
        true => Ok(Some(())),
        false => Err(From::from("Cannot delete object."))
    }
}
