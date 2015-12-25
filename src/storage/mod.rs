mod filesystem;
extern crate rustc_serialize;
extern crate uuid;

use std::path::Path;
use self::rustc_serialize::json;
use self::uuid::Uuid;

pub struct Storable {
    key: StorableKey,
    data: StorableData,
    map: StorableMap
}

pub struct StorableData {
    id: Uuid,
    payload: json::Json
}

pub struct StorableKey {
    id: Uuid,
    payload: json::Json
}

pub struct StorableMap {
    id: String
}

pub trait DataStorage {
    fn new<T: AsRef<Path>>(T) -> Self;
    fn dump(StorableData) -> Result<Uuid, &'static str>;
    fn load(Uuid) -> Result<StorableData, &'static str>;
}

trait KeysStorage {
    fn new(String) -> Self;
    fn dump(StorableKey) -> Result<Uuid, &'static str>;
    fn load(Uuid) -> Result<StorableKey, &'static str>;
}

trait MapsStorage {
    fn new(String) -> Self;
    fn dump(StorableMap) -> Result<(), &'static str>;
    fn load(String) -> Result<StorableMap, &'static str>;
}
