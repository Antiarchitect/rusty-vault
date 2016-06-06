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

pub type StorageResult<T> = Result<T, Box<Error>>;
pub type StorageResultOption<T> = StorageResult<Option<T>>;

pub trait Config {}
pub trait BaseStorage {
    fn dump<T: Encodable>(&self, id: &String, storable: T) -> StorageResult<()>;
    fn load<T: Decodable>(&self, id: &String) -> StorageResultOption<T>;
    fn delete(&self, id: &String) -> StorageResultOption<()>;
}

pub trait MapsStorage: BaseStorage {}
pub trait KeysStorage: BaseStorage {}
pub trait DataStorage: BaseStorage {}