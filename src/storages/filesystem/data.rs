use std::path::Path;

struct Storage {
    path: Path
}

impl DataStorage for Storage {
    fn new(path: String) -> Self {
        self.path = Path::from(String);
    }

    fn dump(data: StorableData) {

    }

    fn load(id: StorableData::Id) {

    }
}
