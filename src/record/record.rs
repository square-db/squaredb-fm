use serde:: {
  Serialize,
  Deserialize,
};
use std::collections:: {
  HashMap
};
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Record {
  inner: HashMap<String,
  String>
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
  fn random_stamp() -> String;
}

impl RecordT for Record {
  fn new(
    inner: HashMap<String, String>
  ) -> Self {
    Record {
      inner
    }
  }

  fn to_record(data: &str) -> Result<Self,
  serde_json::Error> {
    serde_json::from_str(data)
  }


  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

  fn random_stamp() -> String {
    const LENGTH: usize = 100;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..LENGTH)
    .map(|_| {
      let idx = rng.gen_range(0..CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();

    password
  }
}