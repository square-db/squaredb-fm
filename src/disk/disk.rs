// The purpose of this script is to perform read and write operations

// Consider using constants or configuration options for paths
const DB_DIRECTORY: &str = "/sq/dbd/dbs";

use crate::disk::enc:: {
  Encryptor,
  EncryptorTrait
};
use crate::res::create_response::create_response;
use crate::table::table:: {
  Table,
  TableT
};
use crate::fm::fm:: {
  FsApi,
  FsApiTrait
};
use std::collections::HashMap;

pub struct Disk {
  path: String,
  enc: Encryptor,
}

pub trait DiskTrait {
  fn new(key: String, path: String) -> Self;
  fn write_table(&self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>>;
  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>>;
  fn delete_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn write_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn read_database(&self, db: &str) -> Result<Vec<String>,
  HashMap<String,
  String>>;
  fn delete_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn rename_database(&self, db: &str, new_db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn check_existence(&self, path: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn format_path(&self, components: &[&str]) -> String;
  fn decrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>>;
  fn encrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>>;
  fn exist_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn exist_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
}

impl DiskTrait for Disk {
  fn new(key: String, path: String) -> Self {
    Disk {
      enc: Encryptor::new(&key),
      path,
    }
  }

  fn check_existence(
    &self,
    path: &str
  ) -> Result<(),
  HashMap<String,
  String>> {
    FsApi::exist(path)
    .map_err(|err| create_response("400","Error: Not found!", Some(&err.to_string())))
  }

  fn format_path(&self, components: &[&str]) -> String {
    let mut path = String::from(&self.path);
    path.push_str(DB_DIRECTORY);
    for component in components {
      path.push('/');
      path.push_str(component);
    }
    path
  }

  fn decrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>> {
    self.enc
    .decrypt(self.enc.instance.clone(), data)
    .map_err(|err| create_response("500", "Error: Cannot decrypt data. Private key may be changed.", Some(&err.to_string())))
  }

  fn encrypt(&self, data: &String) -> Result<String,
  HashMap<String,
  String>> {
    self.enc
    .encrypt(self.enc.instance.clone(), data)
    .map_err(|err| create_response("500", "Error: Cannot encrypt data.", Some(&err.to_string())))
  }

  fn exist_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db, table_name]);
    self.check_existence(&path).map_err(|_| create_response("400", "Error: Table doesnot exist!", None))
  }

  fn delete_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db, table_name]);
    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|err| create_response("500", "Error: Cannot delete table.", Some(&err.to_string()))))
  }

  fn write_table(&self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db, &table.name, &format!("{}.table", &table.name)]);
    let encrypted_table = self.encrypt(&table.to_string())?;

    FsApi::write(&path, &encrypted_table)
    .map_err(|err| create_response("500", "Error: Failed to write table.", Some(&err.to_string())))
  }

  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>> {
    let path = self.format_path(&[db, table_name, &format!("{}.table",
      &table_name)]);

    self.check_existence(&path)
    .and_then(|_| FsApi::read(&path).map_err(|err| create_response("500", "Error: Cannot read table.", Some(&err.to_string()))))
    .and_then(|d| self.decrypt(&d).and_then(|decrypted_table| Table::to_table(&decrypted_table).map_err(|err| create_response("500", "Error: Cannot deserialize table.", Some(&err.to_string())))))
  }

  fn write_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db]);

    FsApi::create_dir(&path)
    .map_err(|err| create_response("500", "Error: Failed to create database.", Some(&err.to_string())))
  }

  fn read_database(&self, db: &str) -> Result<Vec<String>,
  HashMap<String,
  String>> {
    let path = self.format_path(&[db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::read_dir(&path).map_err(|err| create_response("500", "Error: Failed to read database directory.", Some(&err.to_string()))))
  }

  fn delete_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|err| create_response("500", "Error: Cannot delete database.", Some(&err.to_string()))))
  }

  fn rename_database(&self, db: &str, new_db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db]);
    let new_path = self.format_path(&[new_db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::rename(&path, &new_path).map_err(|err| create_response("500", "Error: Cannot rename database.", Some(&err.to_string()))))
  }
  
  fn exist_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db]);
    self.check_existence(&path).map_err(|_| create_response("400", "Error: Database doesnot exist!", None))
  }
}