use std::fs::create_dir_all;
use std::path::Path

pub struct Data;

impl DataStorage for Data {

    fn new(path: String) -> Self {
        create_dir_all(Path::from(path));
    }

    fn dump(data: StorableData) -> Result {
        
    }

    fn load(id: StorableDataId) {

    }
}
