// The purpose of this script is to perform read and write operations

// Consider using constants or configuration options for paths
pub const DB_DIRECTORY: &str = "sq/dbd";

use std::path:: {
  Path,
  PathBuf
};
use crate::disk::diskenc:: {
  DiskEnc,
  DiskEncT
};
use crate::lsm:: {
  memtable:: {
    Memtable,
    MemtableT
  }
};
use crate::err::err:: {
  FmError
};
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
  pub enc: DiskEnc
}

pub trait DiskTrait {
  fn new(key: String, path: Option<String>) -> Self;
  fn write_table(&mut self, db: &str, table: Table) -> Result<(),
  FmError>;
  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  FmError>;
  fn delete_table(&mut self, db: &str, table_name: &str) -> Result<(),
  FmError>;
  fn write_database(&self, db: &str) -> Result<(),
  FmError>;
  fn read_database(&self, db: &str) -> Result<Vec<String>,
  FmError>;
  fn delete_database(&self, db: &str) -> Result<(),
  FmError>;
  fn rename_database(&self, db: &str, new_db: &str) -> Result<(),
  FmError>;
  fn check_existence(&self, path: && Path) -> Result<(),
  FmError>;
  fn format_path(&self, components: &[&str]) -> PathBuf;
  fn exist_table(&self, db: &str, table_name: &str) -> Result<(),
  FmError>;
  fn exist_database(&self, db: &str) -> Result<(),
  FmError>;
  fn write_record(&mut self, db: &str, table_name: &str, record: Vec<String>) -> Result<(),
  FmError>;
}

impl DiskTrait for Disk {
  fn new(key: String, path: Option<String>) -> Self {
    let encryption_system: DiskEnc = DiskEnc::new(key);
    Disk {
      memory: Memtable::new(),
      path: path.unwrap_or_else( || String::from("data")),
      enc: encryption_system
    }
  }

  fn write_record(&mut self, db: &str, table_name: &str, _record: Vec<String>) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db, table_name]);
    self.check_existence(&path)
    .and_then(|_| {
      let path_string = path.to_string_lossy().to_string();
      if !self.memory.exist_table(&path_string) {
        self.memory.write_table(&path_string);
      }
      if self.memory.is_full(&path_string) {
        if let Some(_data) = self.memory.read_table(&path_string) {
          self.memory.flush_table(&path_string);
        } else {
          return Err(FmError::MemoryReadError);
        }
      }
      Ok(())
    })
  }

  fn check_existence(
    &self,
    path: && Path
  ) -> Result<(),
  FmError> {
    FsApi::exist(*path)
    .map_err(|_| FmError::NotFound)
  }

  fn format_path(&self, components: &[&str]) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(&self.path);
    path.push(DB_DIRECTORY);
    for component in components {
        path.push(component);
    }
    path
}


  fn exist_table(&self, db: &str, table_name: &str) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db, table_name]);
    self.check_existence(&path).map_err(|_| FmError::TableNotFound)
  }

  fn delete_table(&mut self, db: &str, table_name: &str) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db, table_name]);
    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|_| FmError::TableDeletionError))
    .and_then(|_| {
      if self.memory.exist_table(&path.to_string_lossy().to_string()) {
        self.memory.delete_table(&path.to_string_lossy().to_string())
      }
      Ok(())
    })
  }

  fn write_table(&mut self, db: &str, table: Table) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db, &table.name, &format!("n1-{}.ifrm", &table.name)]);
    let encrypted_table = self.enc.encrypt(&table.to_string())?;

    FsApi::write(&path, &encrypted_table)
    .map_err(|_| FmError::TableCreationError)
    .and_then(|_| {
      self.memory.write_table(&self.format_path(&[db, &table.name]).to_string_lossy().to_string());
      Ok(())
    })
  }

  fn read_table(&self, db: &str, table_name: &str) -> Result<Table,
  FmError> {
    let path: &Path = &self.format_path(&[db, table_name, &format!("n1-{}.ifrm",
      &table_name)]);

    self.check_existence(&path)
    .and_then(|_| FsApi::read(&path).map_err(|_| FmError::TableReadError))
    .and_then(|d| self.enc.decrypt(&d).and_then(|decrypted_table| {
      if let Ok(table) = Table::to_table(&decrypted_table) {
        return Ok(table);
      } else {
        return Err(FmError::TableDeserializationError);
      }
    }))
  }

  fn write_database(&self, db: &str) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db]);
    FsApi::create_dir(&path)
    .map_err(|_| FmError::DatabaseCreationError)
  }

  fn read_database(&self, db: &str) -> Result<Vec<String>,
  FmError> {
    let path: &Path = &self.format_path(&[db]);
    self.check_existence(&path)
    .and_then(|_| FsApi::read_dir(&path).map_err(|_| FmError::DatabaseReadError))
  }

  fn delete_database(&self, db: &str) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::ddel(&path, true).map_err(|_| FmError::DatabaseDeletionError))
  }

  fn rename_database(&self, db: &str, new_db: &str) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db]);
    let new_path: &Path = &self.format_path(&[new_db]);

    self.check_existence(&path)
    .and_then(|_| FsApi::rename(&path, &new_path).map_err(|_| FmError::DatabaseRenameError))
  }

  fn exist_database(&self, db: &str) -> Result<(),
  FmError> {
    let path: &Path = &self.format_path(&[db]);
    self.check_existence(&path).map_err(|_| FmError::DatabaseNotFound)
  }

}