pub mod filesystem;
pub mod postgresql;

use std::error::Error;

use uuid::Uuid;
use rustc_serialize::{Decodable, Encodable};

#[derive(RustcDecodable, RustcEncodable)]
pub struct StorableKey {
    pub key: Box<[u8]>,
    pub iv: Box<[u8]>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct StorableData {
    pub ciphertext: Vec<u8>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct StorableMap {
    pub key_id: Uuid,
    pub data_id: Uuid,
    pub tag: Box<[u8]>
}

pub trait BaseStorage {
    fn dump<T: Encodable>(&self, id: &String, storable: T) -> Result<(), Box<Error>>;
    fn load<T: Decodable>(&self, id: &String) -> Result<Option<T>, Box<Error>>;
    fn delete(&self, id: &String) -> Result<Option<()>, Box<Error>>;
}

pub trait MapsStorage: BaseStorage {}
pub trait KeysStorage: BaseStorage {}
pub trait DataStorage: BaseStorage {}