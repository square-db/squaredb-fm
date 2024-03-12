use std::collections:: {
  BTreeMap,
  HashMap
};
use crate::fm::fm:: {
  FsApi,
  FsApiTrait
};
use crate::res::create_response::create_response;


pub struct sstable;

pub trait sstableTrait {
  fn btree_map_to_str(&self, map: &BTreeMap<u64, String>) -> String;
  fn write_to_disk(&self, path: &str, data: BTreeMap<u64, String>) -> Result<(),
  HashMap<String,
  String>>;
}

impl sstableTrait for sstable {
  fn btree_map_to_str(&self, map: &BTreeMap<u64, String>) -> String {
    let mut result = String::new();
    for (key, value) in map.iter() {
      result.push_str(&format!("{}:{}\n", key, value));
    }
    result.trim_end().to_string()
  }

  fn write_to_disk(&self, path: &str, data: BTreeMap<u64, String>) -> Result<(),
  HashMap<String,
  String>> {
    if let Err(_) = FsApi::exist(&(path.to_owned()+&"/data/0")) {
      if let Err(err) = FsApi::create_dir(&(path.to_owned()+"/data/0")) {
        return Err(create_response("500", "OS Error: Writing level 0 folder failed!", Some(&err.to_string())))
      }
    } else {
      if let Ok(files_vec) = FsApi::read_dir(&(path.to_owned()+"/data/0")) {
        let sstable_file_name = files_vec.len();
        if let Err(err) = FsApi::write(&format!("{}/data/0/{}.sstable", &path, sstable_file_name), &self.btree_map_to_str(&data)) {
          return Err(create_response("500", "OS Error: Writing data to Disk failed!", Some(&err.to_string())))
        }
      } else {
        return Err(create_response("500", "OS Error: Could not read level 0 of data!", None))
      }
    }
    Ok(())
  }
}