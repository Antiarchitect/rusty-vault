struct Config {
    path: Path
}

impl DataStorage for Config {

    fn new(path: String) -> Self {
        let fspath = FsPath::from(path);
        create_dir_all(fspath);
    }

    fn dump(data: StorableData) -> Result {
        Ok
    }

    fn load(id: StorableData::Id) {

    }
}
