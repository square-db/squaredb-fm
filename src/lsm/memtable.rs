use std::collections::BTreeMap;
use parking_lot:: {
  Mutex,
  RwLock,
  deadlock
};
use std::time::Duration;
use crate::err::err::FmError;

pub const CAPACITY: usize = 10;

#[derive(Debug)]
pub struct Memtable {
  pub inner: RwLock<BTreeMap<String,
  Mutex<BTreeMap<String,
  Vec<String>>>>>,
}

pub trait MemtableT {
  fn new() -> Self;
  fn exist_table(&self, path: &str) -> bool;
  fn is_full(&self, path: &str) -> bool;
  fn delete_table(&mut self, path: &str);
  fn write_table(&mut self, path: &str);
  fn read_table(&self, path: &str) -> Option<BTreeMap<String,
  Vec<String>>>;
  fn flush_table(&mut self, path: &str) -> Result<(),
  FmError>;
  fn write_record(&mut self, path: &str, record: Vec<String>) -> Result<(),
  FmError>;
}

impl MemtableT for Memtable {
  fn new() -> Self {
    let data: BTreeMap<String,
    Mutex<BTreeMap<String,
    Vec<String>>>> = BTreeMap::new();
    Self {
      inner: RwLock::new(data),
    }
  }

  fn is_full(&self, path: &str) -> bool {
    let lock = self.inner.read();
    if let Some(table) = lock.get(path) {
      table.lock().len() >= CAPACITY
    } else {
      false
    }
  }

  fn read_table(&self, path: &str) -> Option<BTreeMap<String,
  Vec<String>>> {
    let lock = self.inner.read();
    lock.get(path).map(|table| table.lock().clone())
  }

  fn write_table(&mut self, path: &str) {
    let mut lock = self.inner.write();
    lock.insert(path.to_string(), Mutex::new(BTreeMap::new()));
  }

  fn exist_table(&self, path: &str) -> bool {
    let lock = self.inner.read();
    lock.contains_key(path)
  }

  fn delete_table(&mut self, path: &str) {
    let mut lock = self.inner.write();
    lock.remove(path);
  }

  fn flush_table(&mut self, path: &str) -> Result<(),
  FmError> {
    let timeout = Duration::from_millis(10);
    let lock = self.inner.write();
    if let Some(table) = lock.get(path) {
      if let Some(mut inner_lock) = table.try_lock_for(timeout) {
        inner_lock.clear();
        Ok(())
      } else {
        return Err(FmError::LockTimeout);
      }
    } else {
      return Err(FmError::TableNotFoundInMemory);
    }
  }

  fn write_record(&mut self, path: &str, record: Vec<String>) -> Result<(),
  FmError> {
    let timeout = Duration::from_millis(100);
    let lock = self.inner.write();
    if let Some(table) = lock.get(path) {
      if let Some(mut inner_lock) = table.try_lock_for(timeout) {
        inner_lock.insert(record[0].clone(), record);
        return Ok(());
      } else {
        return Err(FmError::LockTimeout);
      }
    } else {
      Err(FmError::TableNotFoundInMemory)
    }
  }
}