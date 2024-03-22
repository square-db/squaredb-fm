use std::collections:: {
  BTreeMap,
  HashMap
};

pub const CAPACITY: usize = 10;

#[derive(Debug, Clone)]
pub struct Memtable {
  pub inner: BTreeMap<String,
    BTreeMap<String,Vec<String>>>,
}

pub trait MemtableT {
  fn new() -> Self;
  fn exist_table(&self, path: &str) -> bool;
  fn is_full(&mut self, path: &str) -> bool;
  fn delete_table(&mut self, path: &str);
  fn write_table(&mut self, path: &str);
  fn read_table(&self, path: &str) -> Option<BTreeMap<String,
  Vec<String>>>;
  fn flush_table(&mut self, path: &str);
  fn write_record(&mut self, path: &str, record: Vec<String>) -> Result<(),
  HashMap<String,
  String>>;
}

impl MemtableT for Memtable {
  fn new() -> Self {
    let data: BTreeMap<String,
    BTreeMap<String,Vec<String>>> = BTreeMap::new();
    Self {
      inner: data
    }
  }

  fn is_full(&mut self, path: &str) -> bool {
    if self.inner.get(path).unwrap().len() >= CAPACITY {
      return true;
    }else {
      return false;
    }
  }

  fn read_table(&self, path: &str) -> Option<BTreeMap<String,
  Vec<String>>> {
    self.inner.get(path).cloned()
  }

  fn write_table(&mut self, path: &str) {
    let table_data: BTreeMap<String,
    Vec<String>> = BTreeMap::new();
    self.inner.insert(path.to_string(), table_data);
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

  fn write_record(&mut self, path: &str, record: Vec<String>) -> Result<(),
  HashMap<String,
  String>> {
    //[Alice, 12, female]
    //[Ahmed, 11, male]
    self.inner.get_mut(path).unwrap().insert(record[0].clone(), record);
    Ok(())
  }
}