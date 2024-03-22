use serde:: {
  Serialize,
  Deserialize
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
  pub data_types: HashMap<String,
  String>,
}

pub trait TableT {
  fn new(
    name: &str,
    row_names: Vec<String>,
    default_values: HashMap<String, String>,
    required_columns: Vec<String>,
    locked_columns: Vec<String>,
    data_types: HashMap<String, String>,
  ) -> Self;

  fn to_table(data: &String) -> Result<Self,
  serde_json::Error> where Self: Sized;
  
  fn to_string(&self) -> String;
  
  fn default_table() -> Self;
}

impl TableT for Table {
  fn new(
    name: &str,
    row_names: Vec<String>,
    default_values: HashMap<String, String>,
    required_columns: Vec<String>,
    locked_columns: Vec<String>,
    data_types: HashMap<String, String>,
  ) -> Self {
    Table {
      name: name.to_string(),
      // Initialize other fields accordingly
      row_names,
      default_values,
      required_columns,
      locked_columns,
      data_types,
    }
  }

  fn to_table(data: &String) -> Result<Self,
  serde_json::Error> {
    serde_json::from_str(data)
  }

  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

  fn default_table() -> Self {
    let name = String::from("default");
    let row_names = Vec::new();
    let default_values = HashMap::new();
    let required_columns = Vec::new();
    let locked_columns = Vec::new();
    let data_types = HashMap::new();

    Table {
      name,
      row_names,
      default_values,
      required_columns,
      locked_columns,
      data_types,
    }
  }
}