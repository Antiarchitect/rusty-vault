use super::*;

pub struct Data;

impl DataStorage for Data {

    fn new(path: String) -> Self {

    }

    fn dump(data: StorableData) -> Result<(), &'static str> {

    }

    fn load(id: String) {

    }
}
