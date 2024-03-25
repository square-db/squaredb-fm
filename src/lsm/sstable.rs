extern crate sstable;
use std::path:: {
  PathBuf
};
use std::collections::BTreeMap;
use bincode;
use crate::err::err::FmError;
use crate::disk::diskenc:: {
  DiskEnc,
  DiskEncT
};
use crate::fm::fm:: {
  FsApi,
  FsApiTrait
};

pub struct SStable {
  pub enc: DiskEnc
}

pub trait SStableTrait {
  fn new(encryption_system: DiskEnc) -> Self;
  fn to_sstable(&self, data: &BTreeMap<u64,
    Vec<String>>, path: &str) -> Result<(),
  FmError>;
}

impl SStableTrait for SStable {
  fn new(encryption_system: DiskEnc) -> Self {
    Self {
      enc: encryption_system
    }
  }

  fn to_sstable(&self, data: &BTreeMap<u64, Vec<String>>, path: &str) -> Result<(),
  FmError> {
    let mut path_buf = PathBuf::from(path);
    path_buf.push("lev0");

    let mut sstable: Vec<(Vec<u8>, Vec<u8>)> = Vec::with_capacity(data.len());
    let mut prev_value: Option<&Vec<String>> = None;

    for (key, value) in data {
      if prev_value.as_deref() != Some(value) {
        let byte_serialized_value = bincode::serialize(value)
        .map_err(|e| FmError::RecordSerializationError(e.to_string()))?;
        let byte_key = key.to_be_bytes().to_vec();
        sstable.push((byte_key, byte_serialized_value));
        prev_value = Some(value);
      }
    }

    FsApi::create_dir(&path_buf)?;

    let files = FsApi::read_dir(&path_buf)?.len();
    let mut file_path = path_buf.clone();
    file_path.push(format!("n1-{}.db", files));
    let dst = FsApi::open_dst(&file_path)?;

    let mut tb = sstable::TableBuilder::new(sstable::Options::default(), dst);
    for (k, v) in sstable {
      tb.add(&k, &v).map_err(|err| FmError::SSTableBuildingError(err.to_string()))?;
    }

    tb.finish().map_err(|err| FmError::SSTableBuildingError(err.to_string()))?;

    Ok(())
  }
}