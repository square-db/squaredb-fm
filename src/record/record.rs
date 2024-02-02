use serde:: {
  Serialize,
  Deserialize
};
use std::collections:: {
  HashMap
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  inner: HashMap<String,
  String>,
  default_values: HashMap<String,
  String>,
  required_columns: Vec<String>,
  locked_columns: Vec<String>,
  data_types: HashMap<String,
  String>
}

pub trait RecordT {
  fn new(
    inner: HashMap<String, String>,
    default_values: HashMap<String,
    String>,
    required_columns: Vec<String>,
    locked_columns: Vec<String>,
    data_types: HashMap<String,
    String>
  ) -> Self;

  fn to_record(data: &String) -> Result<Self,
  serde_json::Error> where Self: Sized;

  fn to_string(&self) -> String;
}

impl RecordT for Record {
  fn new(
    inner: HashMap<String, String>,
    default_values: HashMap<String,
    String>,
    required_columns: Vec<String>,
    locked_columns: Vec<String>,
    data_types: HashMap<String,
    String>
  ) -> Self {
    Record {
      inner,
      default_values,
      required_columns,
      locked_columns,
      data_types
    }
  }

  fn to_record(data: &String) -> Result<Self,
  serde_json::Error> {
    serde_json::from_str(data)
  }

  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}