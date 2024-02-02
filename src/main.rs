pub mod fm;
pub mod disk;
pub mod table;
pub mod res;
pub mod record;

use std::collections::HashMap;
use disk::disk::Disk;
use disk::disk::DiskTrait;
use table::table:: {
  Table,
  TableT
};

fn main() {
  let users_data: Table = Table::new(
    /*name:*/ "users_data",
    /* row_names:*/ Vec::new(),
    /*default_values:*/ HashMap::new(),
    /*required_columns:*/ Vec::new(),
    /*locked_columns:*/ Vec::new(),
    /*data_types: */ HashMap::new(),
  );

  let d: Disk = Disk::new(
    "passw".to_string(),
    "data".to_string()
  );

  // Create Table
  println!("{:?}", d.write_table("users", users_data));

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
}