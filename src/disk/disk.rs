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
use crate::record::record:: {
  Record,
  RecordT
};
use crate::fm::fm:: {
  FsApi,
  FsApiTrait
};
use std::collections::HashMap;
//use rayon::prelude::*;

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
  fn write_record(&self, db: &str, table_name: &str, record: Record) -> Result<(),
  HashMap<String,
  String>>;
  fn encrypt_record(&self, record: Record) -> Result<String,
  String>;
  fn read_record(&self, db:&str, table_name: &str) -> Result<(),HashMap<String,
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
    .map_err(|err| create_response("400", "Error: Not found!", Some(&err.to_string())))
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
    let path: String = self.format_path(&[db, table_name]);
    self.check_existence(&path).map_err(|_| create_response("400", "Error: Table doesnot exist!", None))
  }

  fn delete_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, table_name]);
    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|err| create_response("500", "Error: Cannot delete table.", Some(&err.to_string()))))
  }

  fn write_table(&self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, &table.name, &format!("{}.ifrm", &table.name)]);
    let encrypted_table = self.encrypt(&table.to_string())?;

    FsApi::write(&path, &encrypted_table)
    .map_err(|err| create_response("500", "Error: Failed to write table.", Some(&err.to_string())))
  }

  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, table_name, &format!("{}.ifrm",
      &table_name)]);

    self.check_existence(&path)
    .and_then(|_| FsApi::read(&path).map_err(|err| create_response("500", "Error: Cannot read table.", Some(&err.to_string()))))
    .and_then(|d| self.decrypt(&d).and_then(|decrypted_table| Table::to_table(&decrypted_table).map_err(|err| create_response("500", "Error: Cannot deserialize table.", Some(&err.to_string())))))
  }

  fn write_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db]);

    FsApi::create_dir(&path)
    .map_err(|err| create_response("500", "Error: Failed to create database.", Some(&err.to_string())))
  }

  fn read_database(&self, db: &str) -> Result<Vec<String>,
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::read_dir(&path).map_err(|err| create_response("500", "Error: Failed to read database directory.", Some(&err.to_string()))))
  }

  fn delete_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|err| create_response("500", "Error: Cannot delete database.", Some(&err.to_string()))))
  }

  fn rename_database(&self, db: &str, new_db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db]);
    let new_path = self.format_path(&[new_db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::rename(&path, &new_path).map_err(|err| create_response("500", "Error: Cannot rename database.", Some(&err.to_string()))))
  }

  fn exist_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db]);
    self.check_existence(&path).map_err(|_| create_response("400", "Error: Database doesnot exist!", None))
  }

  fn encrypt_record(&self, record: Record) -> Result<String,
  String> {
    self.encrypt(&record.to_string())
    .and_then(|record| {
      Ok(format!("{} {} \n", Record::random_stamp(), record))
    }).map_err(|_| {
      String::from("Error: Cannot format record!")
    })
  }

  fn write_record(&self, db: &str, table_name: &str, record: Record) -> Result<(),
  HashMap<String,
  String>> {
    let path = self.format_path(&[db, table_name]);

    match self.check_existence(&path) {
      Ok(_) => {
        match FsApi::read_dir(&path) {
          Ok(files) => {
            let last_file = files
            .iter()
            .filter(|file_name| !file_name.ends_with(".ifrm"))
            .last();
            let file_number = files.len();
            let mut file_number_str = file_number.to_string();
            let file_path = self.format_path(&[db, table_name,
              &format!("{}", &last_file.unwrap_or_else( || {
                file_number_str.push_str(".rdb");
                &file_number_str
              }))]);
            println!("{}", &file_path);
            if let Ok(data) = self.encrypt_record(record) {
              if let Some(_) = last_file {
                let write_result = if let Some(file_size) = FsApi::get_f_size(&file_path).ok() {
                  if file_size >= 8192 {
                    FsApi::write(&file_path, &data)
                    .map_err(|err| create_response("500", "Error: Cannot write record", Some(&err)))
                  } else {
                    FsApi::append_file(&file_path, &data)
                    .map_err(|err| create_response("500", "Error: Cannot write record", Some(&err)))
                  }
                } else {
                  Err(create_response("500", "Error: Failed to get file size", None))
                };
                return write_result;
              } else {
                FsApi::write(&file_path, &data)
                .map_err(|err| create_response("500", "Error: Cannot write record", Some(&err)))
              }
            } else {
              Err(create_response("500", "Error: Cannot encrypt record", None))
            }
          }
          Err(err) => return Err(create_response("500", "Error: Failed to read table directory.", Some(&err.to_string()))),
        }
      }
      Err(err) => return Err(err),
    }
  }
  
  fn read_record(&self, db:&str, table_name: &str) -> Result<(),HashMap<String,
  String>> {
    Ok(())
  }
  
}