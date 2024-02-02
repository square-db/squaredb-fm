use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCrypt256;
use std::error::Error;

#[derive(Clone)]
pub struct Encryptor {
  pub instance: MagicCrypt256
}

pub trait EncryptorTrait {
  fn new(key: &str) -> Self;
  fn encrypt(&self, mcrypt: MagicCrypt256, txt: &str) -> Result<String,Box<dyn
  Error>>;
  fn decrypt(&self, mcrypt: MagicCrypt256, encrypted_string: &String) -> Result<String,
  String>;
}

impl EncryptorTrait for Encryptor {
  fn new(key: &str) -> Self {
    Encryptor {
      instance: {
        let mcrypt_init: MagicCrypt256 = new_magic_crypt!(key, 256);
        mcrypt_init
      }
    }
  }

  fn encrypt(&self, mcrypt: MagicCrypt256, txt: &str) -> Result<String,
  Box<dyn
  Error>> {
    let encrypted_txt: String = mcrypt.encrypt_str_to_base64(txt);
    Ok(encrypted_txt)
  }

  fn decrypt(&self, mcrypt: MagicCrypt256, encrypted_string: &String) -> Result<String,
  String> {
    let decrypted_result = mcrypt.decrypt_base64_to_string(&encrypted_string);

    match decrypted_result {
      Ok(decrypt_text) => Ok(decrypt_text),
      Err(err) => Err(err.to_string()),
    }

  }
}