trait StorableData {
    fn id() -> String;
}

trait StorableKey {
    fn id() -> String;
}

trait StorableMap {
    fn id() -> String;
}

pub trait DataStorage {
    fn new(String) -> Self;
    fn dump(StorableData) -> Result;
    fn load(StorableData::Id) -> StorableData;
}

trait KeysStorage {
    fn new(String) -> Self;
    fn dump(StorableKey) -> Result;
    fn load(StorableKey::Id) -> StorableKey;
}

trait MapsStorage {
    fn new(String) -> Self;
    fn dump(StorableMap) -> Result;
    fn load(StorableMap::Id) -> StorableMap;
}
