use crate::record::record:: {
  Record,
  RecordT
};
use crate::disk::enc::Encryptor;
use crate::disk::enc::EncryptorTrait;
use crate::res::create_response::create_response;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct DiskEnc{
  enc: Encryptor
}

pub trait DiskEncT{
  fn new(key: String) -> Self;
  fn encrypt_record(&self, record: Record) -> Result<String,
  HashMap<String,
  String>>;
  fn decrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>>;
  fn encrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>>;
}


impl DiskEncT for DiskEnc{
  fn new(key: String) -> Self {
    DiskEnc {
      enc: Encryptor::new(&key),
    }
  }
  
  fn encrypt_record(&self, record: Record) -> Result<String,
  HashMap<String,
  String>> {
    let formatted_record: String = format!("{}", record.to_string());
    self.encrypt(&formatted_record)
    .and_then(|encrypted_record| {
      Ok(encrypted_record)
    }).map_err(|_| {
      create_response("500" , "Error: Cannot encrypt record!", None)
    })
  }
  
  fn decrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>> {
    self.enc
    .decrypt(self.enc.instance.clone(), data)
    .map_err(|err| create_response("500", "Error: Cannot decrypt data. Data has been either corrupted or key is lost!", Some(&err.to_string())))
  }

  fn encrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>> {
    self.enc
    .encrypt(self.enc.instance.clone(), data)
    .map_err(|err| create_response("500", "Error: Cannot encrypt data.", Some(&err.to_string())))
  }
}