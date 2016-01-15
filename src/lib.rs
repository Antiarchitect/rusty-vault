mod crypt;
mod storage;

use storage as current_storage;

pub fn dump(external_id: String, data: Vec<u8>) -> crypt::EncryptionResult {
    crypt::encrypt(&external_id.into_bytes(), &data)
}

pub struct LoadResult {
    pub data: Vec<u8>
}

trait FakeLoadResult {
    fn new(data: &'static str) -> Self;
    fn data(&self) -> Vec<u8> { self.data() }
}

impl FakeLoadResult for LoadResult {
    fn new(data: &'static str) -> LoadResult {
        LoadResult { data: data.to_string().into_bytes() }
    }
}

pub fn load(external_id: &String) -> LoadResult {
    LoadResult::new("fakevalue")
}
