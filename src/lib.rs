use std::sync::mpsc::channel;
use std::thread;
use std::error::Error;
use std::marker::{Sync, Send};

mod crypt;

extern crate uuid;
use uuid::Uuid;

extern crate rustc_serialize;

mod storages;
use storages::{StorableKey, StorableData, StorableMap};
use storages::{KeysStorage, DataStorage, MapsStorage};

struct Vault<K, D, M>
    where
        K: 'static + KeysStorage + Sync + Send,
        D: 'static + DataStorage + Sync + Send,
        M: 'static + MapsStorage + Sync + Send,
{
    keys: K,
    data: D,
    maps: M
}

impl<K: KeysStorage + Sync + Send, D: DataStorage + Sync + Send, M: MapsStorage + Sync + Send> Vault<K, D, M> {

    pub fn dump(&self, external_id: &String, data: Vec<u8>) -> Result<(), Box<Error>> {
        try!(self.delete(external_id)); // To replace existing object we should remove the previous one.

        let encrypted = crypt::encrypt(external_id.as_bytes(), &data);
        let key_id = Uuid::new_v4();
        let data_id = Uuid::new_v4();

        let (tx, rx) = channel();

        let storable = StorableKey { key: encrypted.key, iv: encrypted.iv };
        let key_tx = tx.clone();
        let keys_storage = self.keys;
        thread::spawn(move || {
            key_tx.send(keys_storage.dump(&key_id.to_string(), storable).map_err(|e| e.to_string()))
        });

        let storable = StorableData { ciphertext: encrypted.ciphertext };
        let data_tx = tx.clone();
        let data_storage = self.data;
        thread::spawn(move || {
            data_tx.send(data_storage.dump(&data_id.to_string(), storable).map_err(|e| e.to_string()))
        });

        let storable = StorableMap { key_id: key_id, data_id: data_id, tag: encrypted.tag };
        let map_tx = tx.clone();
        let map_id = external_id.clone();
        let maps_storage = self.maps;
        thread::spawn(move || {
            map_tx.send(maps_storage.dump(&map_id, storable).map_err(|e| e.to_string()))
        });

        let results = (0..3).map(|_| rx.recv()).collect::<Result<Vec<_>, _>>().unwrap();

        for result in results.into_iter() {
            match result {
                Ok(_) => {},
                Err(e) => return Err(From::from(e.to_string()))
            }
        }
        Ok(())
    }

    pub fn load(&self, external_id: &String) -> Result<Option<Vec<u8>>, Box<Error>> {
        let map: storages::StorableMap = match try!(self.maps.load(external_id)) {
            Some(value) => value,
            None => return Ok(None)
        };

        let (key_tx, key_rx) = channel();
        let id = map.key_id.to_string();
        let keys_storage = self.keys;
        thread::spawn(move || {
            key_tx.send(keys_storage.load(&id).map_err(|e| e.to_string()))
        });

        let (data_tx, data_rx) = channel();
        let id = map.data_id.to_string();
        let data_storage = self.data;
        thread::spawn(move || {
            data_tx.send(data_storage.load(&id).map_err(|e| e.to_string()))
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

    pub fn delete(&self, external_id: &String) -> Result<Option<()>, Box<Error>> {
        let map: storages::StorableMap = match try!(self.maps.load(external_id)) {
            Some(value) => value,
            None => return Ok(None)
        };

        let (tx, rx) = channel();

        let map_tx = tx.clone();
        let id = external_id.clone();
        let maps_storage = self.maps;
        thread::spawn(move || {
            map_tx.send(maps_storage.delete(&id).map_err(|e| e.to_string()))
        });

        let id = map.key_id.to_string();
        let key_tx = tx.clone();
        let keys_storage = self.keys;
        thread::spawn(move || {
            key_tx.send(keys_storage.delete(&id).map_err(|e| e.to_string()))
        });

        let id = map.data_id.to_string();
        let data_tx = tx.clone();
        let data_storage = self.data;
        thread::spawn(move || {
            data_tx.send(data_storage.delete(&id).map_err(|e| e.to_string()))
        });

        let results = (0..3).map(|_| rx.recv()).collect::<Result<Vec<_>, _>>().unwrap();
        match results.into_iter().all(|i| i.is_ok()) {
            true => Ok(Some(())),
            false => Err(From::from("Cannot delete object."))
        }
    }
}