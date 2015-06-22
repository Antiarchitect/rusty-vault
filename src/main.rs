extern crate crypto;
extern crate rand;

use crypto::aes_gcm::AesGcm;
use crypto::aead::AeadEncryptor;
use crypto::aes::KeySize::KeySize256;
use rand::{Rng, OsRng};

fn main() {
    const TAG_LENGTH: usize = 16;
    const KEY_LENGTH: usize = 32;
    const IV_LENGTH: usize = 12;
    let external_id = "e338c3d0-855c-4103-b427-585148b9da34".to_string().into_bytes();
    let data = "secret data".to_string().into_bytes();
    
    let data_len = data.len();
    
    let mut key: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
    let mut iv: [u8; IV_LENGTH] = [0; IV_LENGTH];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    
    let mut cipher = AesGcm::new(KeySize256, &key, &iv, &external_id);
    
    let mut out: Vec<u8> = std::iter::repeat(0).take(data_len).collect();
    let mut tag: [u8; TAG_LENGTH] = [0; TAG_LENGTH];
            
    cipher.encrypt(&data, &mut out, &mut tag);
    
    println!("Data: {:?}", data);
    println!("Cipher {:?}", out);
}
