// clear && cargo build --release && for i in {1..1000}; do clear && target/release/squaredb_fm; done
pub mod fm;
pub mod disk;
pub mod table;
pub mod res;
pub mod record;

use std::time::Instant;
use std::collections::HashMap;
use disk::disk::Disk;
use disk::disk::DiskTrait;
use table::table:: {
Table,
TableT
};
use crate::record::record:: {
Record,
RecordT
};

fn main() {
let start_time = Instant::now();
let users_data: Table = Table::new(
/*name:*/ "users_data",
/* row_names:*/ Vec::new(),
/*default_values:*/ HashMap::new(),
/*required_columns:*/ Vec::new(),
/*locked_columns:*/ Vec::new(),
/*data_types: */ HashMap::new(),
);
let mut data = HashMap::new();
data.insert(String::from("Key"), String::from("Value"));
let record: Record = Record::new(
  data
);

let d: Disk = Disk::new(
"passw".to_string(),
"data".to_string()
);

// Create Table
//println!("{:?}", d.write_table("users", users_data));

// Read Table
//println!("{:?}", d.read_table("users", "users_data"));

// Delete Table
//println!("{:?}", d.delete_table("users", "users_data"));

// Exist Table
//println!("{:?}", d.exist_table("users", "users_data"));

// Read Database
//println!("{:?}", d.read_database("users"));

// Write Database
//println!("{:?}", d.write_database("orders"));

// Rename Database
//println!("{:?}", d.rename_database("orders", "orderd"));

// Delete Database
//println!("{:?}", d.delete_database("orderd"));

// exist Database
//println!("{:?}", d.exist_database("orderd"));

//write records
//println!("{:?}", d.write_record("users", "users_data", record.clone()));

//read records
let records =  d.read_record("users", "users_data").unwrap_or_else(|_| Vec::new() );
println!("{:?}", &records);

//update records
//println!("{:?}", d.update_record(records[1].clone(), record.clone()));

//delete records
println!("{:?}", d.delete_record(records[3].clone()));


let elapsed_time = start_time.elapsed();
println!("Time taken: {:?}", elapsed_time);
}