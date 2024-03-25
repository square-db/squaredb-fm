#![crate_name = "squaredb_fm"]
#![crate_type = "lib"]
extern crate serde;
extern crate sstable;

pub mod fm;
pub mod disk;
pub mod table;
pub mod err;
pub mod lsm;