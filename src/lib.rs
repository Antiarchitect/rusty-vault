mod crypt;

extern crate uuid;
use uuid::Uuid;

extern crate rustc_serialize;

use std::sync::mpsc::channel;
use std::thread;

mod storages;
use storages::filesystem as storage;

pub fn dump(external_id: String, data: Vec<u8>) -> Result<(), String>  {
    let result = crypt::encrypt(external_id.as_bytes(), &data);

    let (key_tx, key_rx) = channel();
    let storable_key = storage::StorableKey { key: result.key, iv: result.iv };
    thread::spawn(move || {
        key_tx.send(storage::store_key(storable_key).unwrap()).unwrap();
    });

    let (data_tx, data_rx) = channel();
    let storable_data = storage::StorableData { ciphertext: result.ciphertext };
    thread::spawn(move || {
        data_tx.send(storage::store_data(storable_data).unwrap()).unwrap();
    });

    let key_id = key_rx.recv().unwrap();
    let data_id = data_rx.recv().unwrap();
    let external_uuid = Uuid::parse_str(&external_id).unwrap();
    storage::store_map(external_uuid, storage::StorableMap { key_id: key_id, data_id: data_id, tag: result.tag })
}

pub fn load(external_id: &String) -> Vec<u8> {
    let map = storage::load_map(&external_id).unwrap();

    let (key_tx, key_rx) = channel();
    let key_id = map.key_id.to_string();
    thread::spawn(move || {
        key_tx.send(storage::load_key(&key_id).unwrap()).unwrap();
    });

    let (data_tx, data_rx) = channel();
    let data_id = map.data_id.to_string();
    thread::spawn(move || {
        data_tx.send(storage::load_data(&data_id).unwrap()).unwrap();
    });

    let key = key_rx.recv().unwrap();
    let data = data_rx.recv().unwrap();

    let result = crypt::decrypt(external_id.as_bytes(), &key.key, &key.iv, &data.ciphertext, &map.tag);

    result.plaintext
}
