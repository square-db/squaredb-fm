use serde:: {
  Serialize,
  Deserialize,
};
use std::collections:: {
  HashMap
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Record {
  pub inner: HashMap<String,
  String>,
  pub read_from: String,
  pub index: isize,
}

pub trait RecordT {
  fn new(
    inner: HashMap<String, String>,
  ) -> Self;

  fn to_record(data: &str) -> Result<Self,
  serde_json::Error>
  where
  Self: Sized;

  fn to_string(&self) -> String;
}

impl RecordT for Record {
  fn new(
    inner: HashMap<String, String>
  ) -> Self {
    Record {
      inner,
      read_from: String::from(""),
      index: -1
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