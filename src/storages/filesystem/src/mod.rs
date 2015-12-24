use std::path::Path;
use std::fs;

pub mod data;
pub mod keys;
pub mod maps;

pub fn create_path(path: String) {
    fs::create_dir_all(string_to_path(path))
}

fn string_to_path(path: String) {
    Path::from(path).ok()
}
