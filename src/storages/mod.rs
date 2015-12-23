mod filesystem;

trait StorableData {
    fn id() -> self::Id;
}

trait StorableKey {
    fn id() -> self::Id;
}

trait StorableMap {
    fn id() -> self::Id;
}

pub trait DataStorage {
    fn dump(StorableData) -> Result;
    fn load(StorableData::Id) -> StorableData;
}

trait KeysStorage {
    fn dump(StorableKey) -> Result;
    fn load(StorableKey::Id) -> StorableKey;
}

trait MapsStorage {
    fn dump(StorableMap) -> Result;
    fn load(StorableMap::Id) -> StorableMap;
}
