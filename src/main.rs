pub mod fm;
pub mod disk;
pub mod table;
pub mod res;

use std::collections::HashMap;
use disk::disk:: {
  Disk,
  DiskTrait
};
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
    /*require_admin_columns: */Vec::new(),
    /*data_types: */ HashMap::new(),
  );

  let d: Disk = Disk::new(
    "passw".to_string(),
    "data".to_string()
  );

  //create Table
  //println!("{:?}", d.wt("users", users_data));

  //read Table
  //println!("{:?}", d.rt("users", "users_data"));
  
  //delete Table
  //println!("{:?}", d.dt("users", "users_data"));
  
  //exsit Table
  //println!("{:?}", d.et("users", "users_data"));
  
  //read Databse
  //println!("{:?}", d.rdb("users"));
  
  //write Databse
  //println!("{:?}", d.wdb("orders"));
  
  //rename Databse
  //println!("{:?}", d.redb("orders", "orderd"));
  
  //delete Databse
  //println!("{:?}", d.dd("orders"));
  
}