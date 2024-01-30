use serde:: {
  Serialize,
  Deserialize
};
use chrono:: {
  Utc,
  DateTime
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
  pub name: String,
  pub row_names: Vec<String>,
  pub default_values: HashMap<String,
  String>,
  pub required_columns: Vec<String>,
  pub locked_columns: Vec<String>,
  pub require_admin_columns: Vec<String>,
  pub data_types: HashMap<String,
  String>,
  pub _stamp: String,
}

pub trait TableT {
  fn new(
    name: &str,
    row_names: Vec<String>,
    default_values: HashMap<String, String>,
    required_columns: Vec<String>,
    locked_columns: Vec<String>,
    require_admin_columns: Vec<String>,
    data_types: HashMap<String, String>,
  ) -> Self;

  fn to_table(data: &String) -> Result<Self,
  serde_json::Error> where Self: Sized;

  fn to_string(&self) -> String;
}

impl TableT for Table {
  fn new(
    name: &str,
    row_names: Vec<String>,
    default_values: HashMap<String, String>,
    required_columns: Vec<String>,
    locked_columns: Vec<String>,
    require_admin_columns: Vec<String>,
    data_types: HashMap<String, String>,
  ) -> Self {
    // Get the current UTC time
    let current_utc_time: DateTime<Utc> = Utc::now();

    // Format the time as a String
    let timestamp_string: String = current_utc_time.format("%Y-%m-%d %H:%M:%S").to_string();

    Table {
      name: name.to_string(),
      // Initialize other fields accordingly
      row_names,
      default_values,
      required_columns,
      locked_columns,
      require_admin_columns,
      data_types,
      _stamp: timestamp_string,
    }
  }

  fn to_table(data: &String) -> Result<Self,
  serde_json::Error> {
    serde_json::from_str(data)
  }

  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}