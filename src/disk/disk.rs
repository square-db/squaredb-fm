/*
The purpose of this script is to perform read and write operations
*/

/*
Table :
- read ×
- write ×
- delete ×
- exsit ×
Database :
- read  ×
- write ×
- delete ×
- exsit ×
- rename ÷
Record   :
- read
- write
*/
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
  fn wt(&self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>>;
  fn rt(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>>;
  fn dt(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn et(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>>;
  // Database
  fn wdb(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn rdb(&self, db: &str) -> Result<Vec<String>,
  HashMap<String,
  String>>;
  fn ed(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn dd(&self, db: &str) -> Result<(),
  HashMap<String,
  String>>;
  fn redb(&self, db: &str, new_db: &str) -> Result<(),
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

  fn dt(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}/{}", &self.path, db, table_name);
    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Table '{}' in database '{}' not found", table_name, db), Some(&err.to_string())))
    .and_then(|_| FsApi::ddel(&path.clone(), true)
      .map_err(|err| create_response("500", &format!("OS Error: Cannot delete Table '{}' in database '{}'", table_name, db), Some(&err.to_string())))
    )
  }

  fn et(&self, db: &str, table_name: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}/{}", &self.path, db, table_name);
    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Table '{}' in database '{}' not found", table_name, db), Some(&err.to_string())))
  }

  fn wt(&self, db: &str, table: Table) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}/{}/{}.table", &self.path, db, &table.name, &table.name);
    let encrypted_table = self.enc.encrypt(self.enc.instance.clone(), &table.to_string());

    FsApi::write(&path, encrypted_table.as_bytes())
    .map_err(|err| create_response("500", &format!("OS Error: Error writing table '{}' in database '{}'", &table.name, db), Some(&err.to_string())))
  }

  fn rt(&self, db: &str, table_name: &str) -> Result<Table,
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}/{}/{}.table", &self.path, db, table_name, table_name);

    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Table '{}' in database '{}' not found", table_name, db), Some(&err.to_string())))
    .and_then(|_| FsApi::read(&path)
      .map_err(|err| create_response("500", &format!("OS Error: Cannot read table '{}' in database '{}'", table_name, db), Some(&err.to_string())))
    )
    .and_then(|d| self.enc.decrypt(self.enc.instance.clone(), &d)
      .map_err(|err| create_response("500", &format!("ENC Error: Cannot decrypt table '{}' in database '{}'. Your PRIVATE_KEY was probably changed!", table_name, db), Some(&err.to_string())))
      .and_then(|decrypted_table| Table::to_table(&decrypted_table)
        .map_err(|err| create_response("500", &format!("Cannot deserialize table '{}' in database '{}'", table_name, db), Some(&err.to_string())))
      )
    )
  }

  fn wdb(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}", &self.path, db);
    FsApi::create_dir(&path)
    .map_err(|err| create_response("500", &format!("OS Error: Error creating database '{}'", db), Some(&err.to_string())))
  }

  fn rdb(&self, db: &str) -> Result<Vec<String>,
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}", &self.path, db);
    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Database '{}' not found", db), Some(&err.to_string())))
    .and_then(|_| FsApi::read_dir(&path)
      .map_err(|err| create_response("500", &format!("OS Error: Error reading database directory '{}'", db), Some(&err.to_string())))
    )
  }

  fn ed(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}", &self.path, db);
    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Database '{}' not found", db), Some(&err.to_string())))
  }

  fn dd(&self, db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}", &self.path, db);
    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Database '{}' not found", db), Some(&err.to_string())))
    .and_then(|_| FsApi::ddel(&path.clone(), true)
      .map_err(|err| create_response("500", &format!("OS Error: Cannot delete Database '{}'", db), Some(&err.to_string())))
    )
  }

  fn redb(&self, db: &str, new_db: &str) -> Result<(),
  HashMap<String,
  String>> {
    let path = format!("{}/sq/dbd/dbs/{}", &self.path, db);
    let new_path = format!("{}/sq/dbd/dbs/{}", &self.path, new_db);
    FsApi::exsit(&path.clone())
    .map_err(|err| create_response("400", &format!("EXSITENCE Error: Database '{}' not found", db), Some(&err.to_string())))
    .and_then(|_| FsApi::rename(&path.clone(), &new_path)
      .map_err(|err| create_response("500", &format!("OS Error: Database '{}' cannot be renamed to '{}'", db, new_db), Some(&err.to_string())))
    )
  }
}