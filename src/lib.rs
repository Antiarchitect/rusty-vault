use std::sync::mpsc::channel;
use std::thread;
use std::error::Error;

mod crypt;

extern crate uuid;
use uuid::Uuid;

extern crate rustc_serialize;

mod storages;
use storages::{StorableKey, StorableData, StorableMap};

use storages::filesystem as fs_storage;
const KEYS_STORAGE: fs_storage::Storage = fs_storage::Storage { path: "/home/andrey/Documents/storages/keys" };

use storages::postgresql as pg_storage;
const DATA_STORAGE: pg_storage::Storage = pg_storage::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_data", table_name: "data" };
const MAPS_STORAGE: pg_storage::Storage = pg_storage::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_maps", table_name: "maps"  };

pub fn dump(external_id: &String, data: Vec<u8>) -> Result<(), Box<Error>>  {

    try!(delete(external_id)); // To replace existing object we should remove the previous one.

    let encrypted = crypt::encrypt(external_id.as_bytes(), &data);
    let key_id = Uuid::new_v4();
    let data_id = Uuid::new_v4();

    let (tx, rx) = channel();

    let storable = StorableKey { key: encrypted.key, iv: encrypted.iv };
    let key_tx = tx.clone();
    thread::spawn(move || {
        let storage = KEYS_STORAGE;
        key_tx.send(storage.dump(&key_id.to_string(), storable).map_err( |e| e.to_string() ))
    });

    let storable = StorableData { ciphertext: encrypted.ciphertext };
    let data_tx = tx.clone();
    thread::spawn(move || {
        let storage = DATA_STORAGE;
        data_tx.send(storage.dump(&data_id.to_string(), storable).map_err( |e| e.to_string() ))
    });

    let storable = StorableMap { key_id: key_id, data_id: data_id, tag: encrypted.tag };
    let map_tx = tx.clone();
    let map_id = external_id.clone();
    thread::spawn(move || {
        let storage = MAPS_STORAGE;
        map_tx.send(storage.dump(&map_id, storable).map_err( |e| e.to_string() ))
    });

    let results = (0..3).map(|_| rx.recv() ).collect::<Result<Vec<_>, _>>().unwrap();

    match results.into_iter().all( |i| i.is_ok() ) {
        true => Ok(()),
        false => Err(From::from("Cannot dump object."))
    }
}

pub fn load(external_id: &String) -> Result<Option<Vec<u8>>, Box<Error>> {
    let storage = MAPS_STORAGE;
    let map: storages::StorableMap = match try!(storage.load(external_id)) {
        Some(value) => value,
        None => return Ok(None)
    };

    let (key_tx, key_rx) = channel();
    let id = map.key_id.to_string();
    let storage = KEYS_STORAGE;
    thread::spawn(move || {
        key_tx.send(storage.load(&id).map_err( |e| e.to_string() ))
    });

    let (data_tx, data_rx) = channel();
    let id = map.data_id.to_string();
    let storage = DATA_STORAGE;
    thread::spawn(move || {
        data_tx.send(storage.load(&id).map_err( |e| e.to_string() ))
    });

    let key: StorableKey = match try!(key_rx.recv().unwrap()) {
        Some(value) => value,
        None => return Err(From::from("Key was not found."))
    };
    let data: StorableData = match try!(data_rx.recv().unwrap()) {
        Some(value) => value,
        None => return Err(From::from("Data was not found."))
    };

    let result = crypt::decrypt(external_id.as_bytes(), &key.key, &key.iv, &data.ciphertext, &map.tag);

    Ok(Some(result.plaintext))
}

pub fn delete(external_id: &String) -> Result<Option<()>, Box<Error>> {
    let storage = MAPS_STORAGE;
    let map: storages::StorableMap = match try!(storage.load(external_id)) {
        Some(value) => value,
        None => return Ok(None)
    };

    let (tx, rx) = channel();

    let map_tx = tx.clone();
    let id = external_id.clone();
    thread::spawn(move || {
        map_tx.send(storage.delete(&id).map_err( |e| e.to_string() ))
    });

    let id = map.key_id.to_string();
    let storage = KEYS_STORAGE;
    let key_tx = tx.clone();
    thread::spawn(move || {
        key_tx.send(storage.delete(&id).map_err( |e| e.to_string() ))
    });

    let id = map.data_id.to_string();
    let storage = DATA_STORAGE;
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
