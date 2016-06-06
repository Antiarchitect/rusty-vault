use std::io::prelude::*;
use std::fs::File;

extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};

use super::VaultResult;
use super::storages::Config as StorageConfig;
pub use super::storages::filesystem::Config as FsStorageConfig;
pub use super::storages::postgresql::Config as PgStorageConfig;

pub struct Config
{
    pub keys: Box<StorageConfig>,
    pub data: Box<StorageConfig>,
    pub maps: Box<StorageConfig>
}

impl Config {

    pub fn from_yaml_file(path: String) -> VaultResult<Self> {
        let mut config_file = try!(File::open(&path));
        let mut config_string = String::new();
        try!(config_file.read_to_string(&mut config_string));
        let yaml = &try!(YamlLoader::load_from_str(&config_string))[0];
        Ok(Config {
            keys: try!(StorageConfig::from_yaml(&yaml["keys"])),
            data: try!(StorageConfig::from_yaml(&yaml["data"])),
            maps: try!(StorageConfig::from_yaml(&yaml["maps"]))
        })
    }

}

impl StorageConfig {

    pub fn from_yaml(yaml: &Yaml) -> VaultResult<Box<Self>> {
        match yaml["adapter"].as_str() {
            Some("filesystem") => Ok(Box::new(FsStorageConfig { path: yaml["path"].as_str().unwrap().to_string() })),
            Some("postgresql") => Ok(Box::new(PgStorageConfig { connection_url: yaml["url"].as_str().unwrap().to_string(), table_name: yaml["table_name"].as_str().unwrap().to_string() })),
            Some(other) => panic!("Unknown adapter: {}", other),
            None => panic!("Adapter is not specified.")
        }
    }

}