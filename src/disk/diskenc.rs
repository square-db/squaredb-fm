use crate::disk::enc::Encryptor;
use crate::disk::enc::EncryptorTrait;
use crate::err::err::{FmError};

#[derive(Debug,Clone)]
pub struct DiskEnc{
  enc: Encryptor
}

pub trait DiskEncT{
  fn new(key: String) -> Self;
  fn decrypt(&self, data: &String) -> Result<String,
  FmError>;
  fn encrypt(&self, data: &String) -> Result<String,
  FmError>;
}


impl DiskEncT for DiskEnc{
  fn new(key: String) -> Self {
    DiskEnc {
      enc: Encryptor::new(&key),
    }
  }
  
  
  fn decrypt(&self, data: &String) -> Result<String,
  FmError> {
    self.enc
    .decrypt(self.enc.instance.clone(), data)
    .map_err(|_| FmError::DecryptionError)
  }

  fn encrypt(&self, data: &String) -> Result<String,
  FmError> {
    self.enc
    .encrypt(self.enc.instance.clone(), data)
    .map_err(|_| FmError::EncryptionError)
  }
}