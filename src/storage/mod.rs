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

pub struct KeysStorageConfig {
    path: Path
}

pub struct DataStorageConfig {
    path: Path
}

pub struct MapsStorageConfig {
    path: Path
}

pub struct StorageConfig<'a> {
    data: &'a DataStorageConfig,
    keys: &'a KeysStorageConfig,
    maps: &'a MapsStorageConfig
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

pub trait Storage {
    fn new(StorageConfig) -> Self;
    fn dump(Storable) -> Result<Uuid, &'static str>;
    fn load(Uuid) -> Result<StorableData, &'static str>;
}

pub trait DataStorage {
    fn new() -> Self;
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
