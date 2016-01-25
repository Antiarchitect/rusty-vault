extern crate crypto;
extern crate rand;

use self::crypto::aes_gcm::AesGcm;
use self::crypto::aead::AeadEncryptor;
use self::crypto::aes::KeySize::KeySize256;
use self::rand::{Rng, OsRng};

const TAG_LENGTH: usize = 16;
const KEY_LENGTH: usize = 32;
const IV_LENGTH: usize = 12;

pub struct EncryptionResult {
  pub key: Box<[u8]>,
  pub iv: Box<[u8]>,
  pub ciphertext: Vec<u8>,
  pub tag: Box<[u8]>
}

pub fn encrypt(auth_data: &[u8], plaintext: &[u8]) -> EncryptionResult {

	let plaintext_len: usize = plaintext.len();

	let mut key: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
	let mut iv: [u8; IV_LENGTH] = [0; IV_LENGTH];
	let mut rng = OsRng::new().ok().unwrap();
	rng.fill_bytes(&mut key);
	rng.fill_bytes(&mut iv);

	let mut cipher = AesGcm::new(KeySize256, &key, &iv, &auth_data);

 	let mut ciphertext: Vec<u8> = Vec::with_capacity(plaintext_len);
	unsafe { ciphertext.set_len(plaintext_len); }
	let mut tag: [u8; TAG_LENGTH] = [0; TAG_LENGTH];

	cipher.encrypt(plaintext, &mut ciphertext, &mut tag);

	return EncryptionResult { key: Box::new(key), iv: Box::new(iv), ciphertext: ciphertext, tag: Box::new(tag) };
}
