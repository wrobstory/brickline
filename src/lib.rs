pub mod inventory;

use crate::inventory::{Color, ItemID};

use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::PathBuf;

pub fn xml_to_string(file_path: &PathBuf) -> Result<String, IOError> {
    let mut file = File::open(file_path)?;
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string)?;
    Ok(xml_string)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ItemColorHashKey {
    item_id: ItemID,
    color: Color,
}
