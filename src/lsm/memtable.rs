use std::collections:: {
  BTreeMap,
  HashMap
};
use crate::disk::diskenc:: {
  DiskEnc,
  DiskEncT
};

pub const CAPACITY: usize = 10;

#[derive(Debug)]
pub struct Memtable {
  pub inner: BTreeMap<String,
  BTreeMap<u64,
  String>>,
  pub enc: DiskEnc
}

pub trait MemtableT {
  fn new(encryption_system: &DiskEnc) -> Self;
  fn exist_table(&self, path: &str) -> bool;
  fn is_full(&mut self, path: &str) -> bool;
  fn delete_table(&mut self, path: &str);
  fn write_table(&mut self, path: &str);
  fn read_table(&self, path: &str) -> Option<BTreeMap<u64,
  String>>;
  fn flush_table(&mut self, path: &str);
  fn write_record(&mut self, path: &str, record: &str) -> Result<(),
  HashMap<String,
  String>>;
}

impl MemtableT for Memtable {
  fn new(encryption_system: &DiskEnc) -> Self {
    let data: BTreeMap<String,
    BTreeMap<u64,
    String>> = BTreeMap::new();
    Self {
      inner: data,
      enc: encryption_system.clone()
    }
  }

  fn is_full(&mut self, path: &str) -> bool {
    if self.inner.get(path).unwrap().len() >= CAPACITY {
      return true;
    }else {
      return false;
    }
  }

  fn read_table(&self, path: &str) -> Option<BTreeMap<u64,
  String>> {
    self.inner.get(path).cloned()
  }

  fn write_table(&mut self, path: &str) {
    let table_data: BTreeMap<u64,
    String> = BTreeMap::new();
    &mut self.inner.insert(path.to_string(), table_data);
  }

  fn exist_table(&self, path: &str) -> bool {
    if let Some(_) = self.inner.get(path) {
      return true;
    }else {
      return false;
    }
  }

  fn delete_table(&mut self, path: &str) {
    self.inner.remove(path);
  }
  
  fn flush_table(&mut self, path: &str) {
    self.inner.insert((&path).to_string(), BTreeMap::new());
  }

  fn write_record(&mut self, path: &str, record: &str) -> Result<(),
  HashMap<String,
  String>> {
    // Generate the next key
    let next_key = match self.inner.get_mut(path).unwrap().keys().last() {
      Some(&max_key) => max_key + 1,
      None => 1,
    };
    self.inner.get_mut(path).unwrap().insert(next_key, record.to_string());
    Ok(())
  }
}