pub mod filesystem;

use uuid::Uuid;

pub struct StorableKey {
    pub key: Box<[u8]>,
    pub iv: Box<[u8]>
}

pub struct StorableData {
    pub ciphertext: Vec<u8>
}

pub struct StorableMap {
    pub key_id: Uuid,
    pub data_id: Uuid,
    pub tag: Box<[u8]>
}
