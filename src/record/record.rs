use serde:: {
  Serialize,
  Deserialize,
};
use std::collections:: {
  BTreeMap
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Record {
  pub inner: BTreeMap<String,
  String>
}

pub trait RecordT {
  fn new(
    inner: BTreeMap<String, String>,
  ) -> Self;

  fn to_record(data: &str) -> Result<Self,
  serde_json::Error>
  where
  Self: Sized;

  fn to_string(&self) -> String;
}

impl RecordT for Record {
  fn new(
    inner: BTreeMap<String, String>
  ) -> Self {
    Record {
      inner: inner.clone()
    }
  }
  
  fn to_record(data: &str) -> Result<Self,
  serde_json::Error> {
    serde_json::from_str(data)
  }


  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
  
}