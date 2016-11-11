use std::error::Error;
use std::marker::{Sync, Send};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

extern crate rustc_serialize;
extern crate uuid;
use uuid::Uuid;

mod crypt;
pub mod config;
use config::Config;

mod storages;
use storages::{StorableKey, StorableData, StorableMap};
use storages::{KeysStorage, DataStorage, MapsStorage};

pub type VaultResult<T> = Result<T, Box<Error>>;
pub type VaultResultOption<T> = VaultResult<Option<T>>;

pub struct Vault<K, D, M>
    where
        K: 'static + KeysStorage + Sync + Send,
        D: 'static + DataStorage + Sync + Send,
        M: 'static + MapsStorage + Sync + Send,
{
    pub keys: Arc<K>,
    pub data: Arc<D>,
    pub maps: Arc<M>
}

impl<K: KeysStorage + Sync + Send, D: DataStorage + Sync + Send, M: MapsStorage + Sync + Send> Vault<K, D, M> {

    pub fn new(keys: K, data: D, maps: M) -> Self {
        Vault { keys: Arc::new(keys), data: Arc::new(data), maps: Arc::new(maps) }
    }

    pub fn from_config(config: &Config) -> Self {
        let keys = KeysStorage::from_config(&config.keys);
        let data = DataStorage::from_config(&config.data);
        let maps = MapsStorage::from_config(&config.maps);
        Vault::new(keys, data, maps)
    }

    pub fn dump(&self, external_id: &String, data: Vec<u8>) -> VaultResult<()> {
        self.delete(external_id)?; // To replace existing object we should remove the previous one.

        let encrypted = crypt::encrypt(external_id.as_bytes(), &data);
        let key_id = Uuid::new_v4();
        let data_id = Uuid::new_v4();

        let (tx, rx) = channel();

        let storable = StorableKey { key: encrypted.key, iv: encrypted.iv };
        let key_tx = tx.clone();
        let keys_storage = self.keys.clone();
        thread::spawn(move || {
            key_tx.send(keys_storage.dump(&key_id.to_string(), storable).map_err(|e| e.to_string()))
        });

        let storable = StorableData { ciphertext: encrypted.ciphertext };
        let data_tx = tx.clone();
        let data_storage = self.data.clone();
        thread::spawn(move || {
            data_tx.send(data_storage.dump(&data_id.to_string(), storable).map_err(|e| e.to_string()))
        });

        let storable = StorableMap { key_id: key_id, data_id: data_id, tag: encrypted.tag };
        let map_tx = tx.clone();
        let map_id = external_id.clone();
        let maps_storage = self.maps.clone();
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

    pub fn load(&self, external_id: &String) -> VaultResultOption<Vec<u8>> {
        let map: storages::StorableMap = match self.maps.load(external_id)? {
            Some(value) => value,
            None => return Ok(None)
        };

        let (key_tx, key_rx) = channel();
        let id = map.key_id.to_string();
        let keys_storage = self.keys.clone();
        thread::spawn(move || {
            key_tx.send(keys_storage.load(&id).map_err(|e| e.to_string()))
        });

        let (data_tx, data_rx) = channel();
        let id = map.data_id.to_string();
        let data_storage = self.data.clone();
        thread::spawn(move || {
            data_tx.send(data_storage.load(&id).map_err(|e| e.to_string()))
        });

        let key: StorableKey = match key_rx.recv().unwrap()? {
            Some(value) => value,
            None => return Err(From::from("Key was not found."))
        };
        let data: StorableData = match data_rx.recv().unwrap()? {
            Some(value) => value,
            None => return Err(From::from("Data was not found."))
        };

        let result = crypt::decrypt(external_id.as_bytes(), &key.key, &key.iv, &data.ciphertext, &map.tag);

        Ok(Some(result.plaintext))
    }

    pub fn delete(&self, external_id: &String) -> VaultResultOption<()> {
        let map: storages::StorableMap = match self.maps.load(external_id)? {
            Some(value) => value,
            None => return Ok(None)
        };

        let (tx, rx) = channel();

        let map_tx = tx.clone();
        let id = external_id.clone();
        let maps_storage = self.maps.clone();
        thread::spawn(move || {
            map_tx.send(maps_storage.delete(&id).map_err(|e| e.to_string()))
        });

        let id = map.key_id.to_string();
        let key_tx = tx.clone();
        let keys_storage = self.keys.clone();
        thread::spawn(move || {
            key_tx.send(keys_storage.delete(&id).map_err(|e| e.to_string()))
        });

        let id = map.data_id.to_string();
        let data_tx = tx.clone();
        let data_storage = self.data.clone();
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