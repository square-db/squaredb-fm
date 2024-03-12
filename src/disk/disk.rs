// The purpose of this script is to perform read and write operations

// Consider using constants or configuration options for paths
pub const DB_DIRECTORY: &str = "/sq/dbd/dbs";

use std::collections::HashMap;
use crate::record::record:: {
  Record,
  RecordT
};
use crate::disk::diskenc:: {
  DiskEnc,
  DiskEncT
};
use crate::lsm:: {
  sstable:: {
    sstableTrait,
    sstable
  },
  memtable:: {
    Memtable,
    MemtableT
  }
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

pub struct Disk {
  pub path: String,
  pub memory: Memtable,
  pub enc: DiskEnc,
}

pub trait DiskTrait {
  fn new(key: String, path: Option<String>) -> Self;
  fn write_table(&mut self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>>;
  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>>;
  fn delete_table(&mut self, db: &str, table_name: &str) -> Result<(),
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
  fn exist_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn exist_database(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn write_record(&mut self, db: &str, table_name: &str, record: Record) -> Result<(),
  HashMap<String,
  String>>;
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
  fn new(key: String, path: Option<String>) -> Self {
    let encryption_system: DiskEnc = DiskEnc::new(key);
    Disk {
      memory: Memtable::new(&encryption_system),
      enc: encryption_system.clone(),
      path: path.unwrap_or_else( || String::from("data"))
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

  fn exist_table(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, table_name]);
    self.check_existence(&path).map_err(|_| create_response("400", "Error: Table doesnot exist!", None))
  }

  fn delete_table(&mut self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, table_name]);
    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|err| create_response("500", "Error: Cannot delete table.", Some(&err.to_string()))))
    .and_then(|_| {
      if self.memory.exist_table(&path) {
        self.memory.delete_table(&path)
      }
      Ok(())
    })
  }

  fn write_table(&mut self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, &table.name, &format!("{}.ifrm", &table.name)]);
    let encrypted_table = self.enc.encrypt(&table.to_string())?;

    FsApi::write(&path, &encrypted_table)
    .map_err(|err| create_response("500", "Error: Failed to write table.", Some(&err.to_string())))
    .and_then(|_| {
      self.memory.write_table(&self.format_path(&[db, &table.name]));
      Ok(())
    })
  }

  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, table_name, &format!("{}.ifrm",
      &table_name)]);

    self.check_existence(&path)
    .and_then(|_| FsApi::read(&path).map_err(|err| create_response("500", "Error: Cannot read table.", Some(&err.to_string()))))
    .and_then(|d| self.enc.decrypt(&d).and_then(|decrypted_table| Table::to_table(&decrypted_table).map_err(|err| create_response("500", "Error: Cannot deserialize table.", Some(&err.to_string())))))
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

  fn write_record(&mut self, db: &str, table_name: &str, record: Record) -> Result<(),
  HashMap<String,
  String>> {
    let path: String = self.format_path(&[db, table_name]);
    self.check_existence(&path)
    .and_then(|_| {
      match self.enc.encrypt_record(record) {
        Ok(encrypted_record) => {
          if !self.memory.exist_table(&path) {
            self.memory.write_table(&path);
          }
          if self.memory.is_full(&path) {
            if let Some(data) = self.memory.read_table(&path) {
              if let Err(err) = sstable.write_to_disk(&path, data) {
                return Err(err);
              }
              self.memory.flush_table(&path);
            } else {
              //Need to be tested//
              return Err(create_response("500", "Cannot load data from Memory", None));
              ///////////////////////
            }
          }
          self.memory.write_record(&path, &encrypted_record)
        },
        Err(err) => Err(err)
      }
    })
  }

  fn read_record(&self, db: &str, table_name: &str) -> Result<Vec<Record>,
  HashMap<String,
  String>> {
    Err(HashMap::new())
  }

  fn update_record(&self, record: Record, record_new: Record) -> Result<(),
  HashMap<String,
  String>> {
    Ok(())
  }

  fn delete_record(&self, record: Record) -> Result<(),
  HashMap<String,
  String>> {
    Ok(())
  }
}