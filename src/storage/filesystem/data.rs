use super::super::DataStorage;
use super::super::StorableData;
use super::super::StorableDataId;

pub struct Data;

impl DataStorage for Data {

    fn new(path: String) -> Self {
        super::create_path(path)
    }

    fn dump(data: StorableData) -> Result {
        Ok
    }

    fn load(id: StorableDataId) {

    }
}
