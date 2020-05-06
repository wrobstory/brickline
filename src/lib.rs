pub mod inventory;

use crate::inventory::{Inventory, Item, Color, ItemID};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::PathBuf;

pub fn xml_to_string(file_path: &PathBuf) -> Result<String, IOError> {
    let mut file = File::open(file_path)?;
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string)?;
    Ok(xml_string)
}

fn build_item_color_hash_table(inventory: &Inventory) -> HashMap<ItemColorHashKey, &Item> {
    inventory.items.iter().fold(HashMap::new(), |mut acc, item| {
        let item_color_key = ItemColorHashKey { item_id: &item.item_id, color: &item.color };
        acc.insert(item_color_key, item);
        acc
    })
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ItemColorHashKey<'a> {
    item_id: &'a ItemID,
    color: &'a Option<Color>,
}
