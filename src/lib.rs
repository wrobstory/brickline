//! Bricktools
//!
//! A small set of tools to manipulate Bricklink wanted lists and perform
//! price analysis

pub mod inventory;

use crate::inventory::{Color, Inventory, Item, ItemID, MinQty};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::PathBuf;

/// Given a path to an XML file, load that file to a String
///
/// # Arguments
///
/// * `file_path`: path to an XML file
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use bricktools::xml_to_string;
///
/// let path = PathBuf::from("/home/user/path/to/file.xml");
/// let xml_string = xml_to_string(&path);
/// ```
pub fn xml_to_string(file_path: &PathBuf) -> Result<String, IOError> {
    let mut file = File::open(file_path)?;
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string)?;
    Ok(xml_string)
}

/// Given an Inventory, build a HashMap of each Inventory Item where
/// the hash key is the ItemID and Color combination for the Item.
/// Note: we explicitly .clone the Item for this map, as we're going to
/// use it as the base case for our merged list.
///
/// # Arguments
///
/// * `inventory`: Bricklink inventory as deserialized from XML
///
/// Example
///
///
/// use bricktools::{xml_to_string, build_item_color_hashmap};
/// use quick_xml::de::from_str;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("/home/user/path/to/file.xml");
/// let xml_string = xml_to_string(&path).unwrap();
/// let inventory = from_str(&xml_string).unwrap();
/// let hm = self::build_item_color_hashmap(&inventory);
///
fn build_item_color_hashmap(inventory: &Inventory) -> HashMap<ItemColorHashKey, Item> {
    inventory
        .items
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            let item_color_key = ItemColorHashKey {
                item_id: &item.item_id,
                color: &item.color,
            };
            // Cloning here as we're going to mutate these
            // Items to combine them with other lists
            acc.insert(item_color_key, item.clone());
            acc
        })
}

fn increment_item(item_to_increment: &mut Item, incrementing_item: &Item) -> () {
    let incrementing_min_qty = match &incrementing_item.min_qty {
        Some(qty) => qty.0,
        None => 1,
    };

    match &item_to_increment.min_qty {
        Some(qty) => {
            item_to_increment.min_qty = Some(MinQty(qty.0 + incrementing_min_qty))
        }
        None => item_to_increment.min_qty = Some(MinQty(incrementing_min_qty)),
    }
}

fn merge_inventories(left_inventory: &Inventory, right_inventory: &Inventory) -> Inventory {
    let (long_inv, short_inv) = if left_inventory.items.len() > right_inventory.items.len() {
        (left_inventory, right_inventory)
    } else {
        (right_inventory, left_inventory)
    };

    let mut long_inv_map = build_item_color_hashmap(long_inv);
    short_inv
        .items
        .iter()
        .fold(&mut long_inv_map, |acc, short_item| {
            let item_color_key = ItemColorHashKey {
                item_id: &short_item.item_id,
                color: &short_item.color,
            };
            if let Some(long_item) = acc.get_mut(&item_color_key) {
                increment_item(long_item, short_item);
            } else {
                acc.insert(item_color_key, short_item.clone());
            }
            acc
        });
    Inventory {
        items: long_inv_map.values().cloned().collect(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ItemColorHashKey<'a> {
    item_id: &'a ItemID,
    color: &'a Option<Color>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::inventory::ItemType;

    #[test]
    fn test_build_item_color_hashmap() {
        let item_1 = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3622")),
            Some(Color(11)),
        );
        let item_1a = item_1.clone();
        let item_2 = Item::build_test_item(ItemType::Part, ItemID(String::from("3039")), None);
        let item_2a = item_2.clone();
        let item_3 =
            Item::build_test_item(ItemType::Part, ItemID(String::from("3001")), Some(Color(5)));
        let inventory = Inventory {
            items: vec![item_1, item_2, item_3],
        };
        let hm = build_item_color_hashmap(&inventory);
        assert_eq!(hm.len(), 3);
        let key_1 = ItemColorHashKey {
            item_id: &ItemID(String::from("3622")),
            color: &Some(Color(11)),
        };
        let key_2 = ItemColorHashKey {
            item_id: &ItemID(String::from("3039")),
            color: &None,
        };
        println!("IS IT TRUE: {}", key_1 > key_2);
        assert_eq!(hm.get(&key_1), Some(&item_1a));
        assert_eq!(hm.get(&key_2), Some(&item_2a));
    }
}
