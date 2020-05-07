//! Bricktools
//!
//! A small set of tools to manipulate Bricklink wanted lists and perform
//! price analysis

pub mod inventory;

use crate::inventory::{Color, Inventory, Item, ItemID, ItemType};

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
///
/// # Arguments
///
/// * `inventory`: Bricklink inventory as deserialized from XML
///
/// # Example
///
/// ```
/// use quick_xml::de::from_str;
/// use std::path::PathBuf;
/// 
/// let path = PathBuf::from("/home/user/path/to/file.xml");
/// let xml_string = xml_to_string(&path).unwrap();
/// let inventory = from_str(&xml_string).unwrap();
/// let hm = build_item_color_hashmap(&inventory);
/// ```
fn build_item_color_hashmap(inventory: &Inventory) -> HashMap<ItemColorHashKey, &Item> {
    inventory
        .items
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            let item_color_key = ItemColorHashKey {
                item_id: &item.item_id,
                color: &item.color,
            };
            acc.insert(item_color_key, item);
            acc
        })
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ItemColorHashKey<'a> {
    item_id: &'a ItemID,
    color: &'a Option<Color>,
}

#[cfg(test)]
mod tests {

    use super::*;

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
        assert_eq!(hm.get(&key_1), Some(&&item_1a));
        assert_eq!(hm.get(&key_2), Some(&&item_2a));
    }
}
