mod crypt;
mod storage;

use storage::filesystem as Storage;

fn main() {
    let external_id = "e338c3d0-855c-4103-b427-585148b9da34".to_string().into_bytes();
    let data = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string().into_bytes();

    let data_storage = Storage::DataStorage.new("./data");
    // let keys_storage = Storage::KeysStorage.new("./keys");
    // let maps_storage = Storage::MapsStorage.new("./maps");

    let result = crypt::encrypt(&external_id, &data);

    println!("Iv: {:?}", result.key.iv);
    println!("Key: {:?}", result.key.key);
    println!("Ciphertext: {:?}", result.ciphertext);
    println!("Tag: {:?}", result.tag);
}
