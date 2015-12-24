extern crate uuid;
use self::uuid::Uuid;

trait StorableData {
    fn id() -> Uuid;
}

trait StorableKey {
    fn id() -> Uuid;
}

trait StorableMap {
    fn id() -> String;
}

pub trait DataStorage {
    fn new(String) -> Self;
    fn dump(StorableData) -> Result<Uuid, &'static str>;
    fn load(String) -> Result<StorableData, &'static str>;
}

trait KeysStorage {
    fn new(String) -> Self;
    fn dump(StorableKey) -> Result<Uuid, &'static str>;
    fn load(Uuid) -> Result<StorableKey, &'static str>;
}

trait MapsStorage {
    fn new(String) -> Self;
    fn dump(StorableMap) -> Result<(), &'static str>;
    fn load(Uuid) -> Result<StorableMap, &'static str>;
}
