// The purpose of this script is to perform read and write operations

// Consider using constants or configuration options for paths
const DB_DIRECTORY: &str = "/sq/dbd/dbs";
const MAX_RECORDS_NUMBER: usize = usize::MAX;
const MAX_FILE_SIZE: u64 = 524_288;

use std::collections::HashMap;
use tokio::task;
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
  fn read_record(&self, db: &str, table_name: &str) -> Result<Vec<Record>,
  HashMap<String,
  String>>;
  fn update_record(&self, record: Record, record_new: Record) -> Result<(),
  HashMap<String,
  String>>;
  fn delete_record(&self, record: Record) -> Result<(),
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
    .map_err(|err| create_response("500", "Error: Cannot decrypt data. Data has been either corrupted or key is lost!", Some(&err.to_string())))
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
    let formatted_record: String = format!("{}", record.to_string());
    self.encrypt(&formatted_record)
    .and_then(|encrypted_record| {
      Ok(encrypted_record)
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
            if file_number == MAX_RECORDS_NUMBER {
              return Err(create_response("400", "Error: Max size of files exceeded!", Some("You cannot add more records to this table use/create anthor table instead!")))
            }
            let mut file_number_str = file_number.to_string();
            let file_path = self.format_path(&[db, table_name,
              &format!("{}", &last_file.unwrap_or_else( || {
                file_number_str.push_str(".rdb");
                &file_number_str
              }))]);
            if let Ok(data) = self.encrypt_record(record) {
              if let Some(_) = last_file {
                let write_result = if let Some(file_size) = FsApi::get_f_size(&file_path).ok() {
                  if file_size >= MAX_FILE_SIZE {
                    let new_file_path: String = self.format_path(&[db, table_name, &format!("{}.rdb", &file_number_str)]);
                    FsApi::write(&new_file_path, &data)
                    .map_err(|err| create_response("500", "Error: Cannot write record", Some(&err)))
                  } else {
                    FsApi::append_file(&file_path, &("\n".to_owned()+&data))
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

  #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
  async fn read_record(&self, db: &str, table_name: &str) -> Result<Vec<Record>,
  HashMap<String,
  String>> {
    let mut records_vec: Vec<Record> = Vec::new();
    let path = self.format_path(&[db, table_name]);

    match self.check_existence(&path) {
      Ok(_) => {
        match FsApi::read_dir(&path) {
          Ok(files) => {
            let record_files: Vec<_> = files
            .iter()
            .filter(|file_name| !file_name.ends_with(".ifrm"))
            .collect();

            let file_paths: Vec<String> = record_files
            .iter()
            .map(|record_file| self.format_path(&[db, table_name, record_file]))
            .collect();

            let tasks = file_paths
            .clone()
            .into_iter()
            .map(|file_path| task::spawn(tokio::fs::read(file_path)))
            .collect::<Vec<_>>();

            let results: Vec<_> = futures::future::try_join_all(tasks)
            .await
            .map_err(|err| create_response("500", "Error: Failed to read files.", Some(&err.to_string())))?;

            for (result, file_path) in results.into_iter().zip(file_paths) {
              let content = match result {
                Ok(data) => data,
                Err(_) => return Err(create_response("500", "Error: Failed to read files.", None))
              };
              let strings: Vec<String> = content
              .split(|&byte| byte == 10)
              .map(|subvec| String::from_utf8_lossy(subvec).to_string())
              .collect();
              let mut record_index = 0; // Initialize record index for each file
              for string in strings {
                if let Ok(data) = self.decrypt(&string) {
                  if let Ok(mut record_struct) = Record::to_record(&data) {
                    record_struct.read_from = file_path.clone();
                    record_struct.index = record_index.clone();
                    records_vec.push(record_struct);
                    record_index += 1; // Increment record index for each record
                  } else {
                    return Err(create_response("500", "Error: Failed to deserialize records.", Some(&data.to_string())));
                  }
                } else {
                  return Err(create_response("500", "Error: Failed to decrypt records.", Some(&string)));
                }
              }
            }
            Ok(records_vec)
          }
          Err(err) => Err(create_response("500", "Error: Failed to read table directory.", Some(&err.to_string()))),
        }
      }
      Err(err) => Err(err),
    }
  }

  fn update_record(&self, record: Record, record_new: Record) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = record.read_from;
    if let Err(err) = self.check_existence(&path) {
      return Err(err);
    }
    let index: isize = record.index;
    if path.is_empty() || index == -1 {
      return Err(create_response("500", "Error: Failed to read record.", None));
    }
    match self.encrypt_record(record_new) {
      Ok(enc_record) => {
        let binding = enc_record.to_string();
        let enc_new_record: Vec<u8> = binding.as_bytes().to_vec();
        if let Err(err) = FsApi::update_file(&path, index as usize, enc_new_record) {
          return Err(create_response("500", "Error: Failed to update record.", Some(&err.to_string())));
        }
        Ok(())
      },
      Err(err) => Err(create_response("500", "Error: Failed to read record.", Some(&err.to_string())))
    }
  }

  fn delete_record(&self, record: Record) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = record.read_from;
    let index: isize = record.index;
    if let Err(err) = self.check_existence(&path) {
      return Err(err);
    }
    if path.is_empty() || index == -1 {
      return Err(create_response("500", "Error: Failed to read record.", None));
    }
     if let Err(err) = FsApi::delete_content_in_file(&path, index as usize) {
      return Err(create_response("500", "Error: Failed to update record.", Some(&err.to_string())));
    }
    Ok(())
  }
}