#![crate_name = "squaredb_fm"]
#![crate_type = "lib"]
extern crate serde;

pub mod fm;
pub mod disk;
pub mod table;
pub mod err;
pub mod lsm;