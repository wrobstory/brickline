//! Bricktools
//!
//! A small set of tools to manipulate Bricklink wanted lists and perform
//! price analysis

pub mod inventory;

use crate::inventory::{Color, Inventory, Item, ItemID, MinQty};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::PathBuf;

use clap::{Arg, App};

/// The primary key of an Inventory Item
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ItemColorHashKey<'a> {
    item_id: &'a ItemID,
    color: &'a Option<Color>,
}

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
fn build_item_color_hashmap(inventory: &Inventory) -> BTreeMap<ItemColorHashKey, Item> {
    inventory
        .items
        .iter()
        .fold(BTreeMap::new(), |mut acc, item| {
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

/// Given two items, add the MinQty of the righthand (incrementing) Item to the
/// lefthand (to-be-incremented) Item. The lefthand item_to_increment *will*
/// be mutated.
///
/// # Arguments
///
/// * `item_to_increment`: Item to be incremented
/// * `incrementing_item`: Item to increment from
///
/// Example
///
/// use bricktools::increment_item;
/// use bricktools::inventory::Item;
///
/// let mut left_item = Item::build_test_item(ItemType::Part, ItemID(String::from("3039")), Some(Color(5)), Some(MinQty(20)));
/// let right_item = Item::build_test_item(ItemType::Part, ItemID(String::from("3039")), Some(Color(5)), Some(MinQty(10)));

/// increment_item(&mut left_item, &right_item);
///
fn increment_item(item_to_increment: &mut Item, incrementing_item: &Item) -> () {
    let incrementing_min_qty = match &incrementing_item.min_qty {
        Some(qty) => qty.0,
        None => 1,
    };

    match &item_to_increment.min_qty {
        Some(qty) => item_to_increment.min_qty = Some(MinQty(qty.0 + incrementing_min_qty)),
        None => item_to_increment.min_qty = Some(MinQty(1 + incrementing_min_qty)),
    }
}

/// Given two Inventories, merge the right inventory into the left one.
/// Here's how the merge happens:
/// 1. Build hash table from left inventory
/// 2. Iterate through right inventory and probe table for ItemId/Color keys
/// 3. If a key is found, add the MinQty of the right inventory to the left.
///    NOTE: The metadata from the *left* inventory is retained. There is no
///    other metadata merging other than MinQty.
/// 4. If no key is found, add the Item from the right inventory to the hash table
/// 5. Convert the .values() of the hash table into .items of a new Inventory
///
/// # Arguments
///
/// * `left_inventory`: Inventory to be merged into
/// * `right_inventory`: Inventory to merge into left inventory
///
/// Example
///
/// use bricktools::merge_inventories;
/// use bricktools::inventory::{Inventory, Item};
///
/// let item = Item::build_test_item(
///       ItemType::Part,
///       ItemID(String::from("3039")),
///       Some(Color(5)),
///       Some(MinQty(20)),
/// );
///
/// let left_inventory = Inventory { items: vec![item] };
/// let right_inventory = Inventory { items: vec![item] };
///
/// let merged_inventory = merge_inventories(left_inventory, right_inventory);
///
pub fn merge_inventories(left_inventory: &Inventory, right_inventory: &Inventory) -> Inventory {
    let mut left_inv_map = build_item_color_hashmap(left_inventory);
    right_inventory
        .items
        .iter()
        .fold(&mut left_inv_map, |acc, right_item| {
            let item_color_key = ItemColorHashKey {
                item_id: &right_item.item_id,
                color: &right_item.color,
            };
            if let Some(left_item) = acc.get_mut(&item_color_key) {
                increment_item(left_item, right_item);
            } else {
                acc.insert(item_color_key, right_item.clone());
            }
            acc
        });
    Inventory {
        items: left_inv_map.values().cloned().collect(),
    }
}

/// CLI Tooling

fn main() {
    let commands = App::new("Bricktools")
        .version("0.1")
        .author("Rob Story")
        .about("Bricklink wanted list helper tools")
        .subcommand(App::new("merge")
            .about("Merges two Bricklink wanted lists")
            .arg(Arg::with_name("left")
                .short('l')
                .required(true)
                .takes_value(true)
                .about("Path to lefthand wanted list, will have right merged into it"))
            .arg(Arg::with_name("right")
                .short('r')
                .required(true)
                .takes_value(true)
                .about("Path to righthand wanted list, will be merged into left")))
        .get_matches();

    match commands.subcommand() {
        ("merge", Some(merge_args)) => {
            println!("LEFT! {:?}", merge_args.value_of("left"));
            println!("RIGHT {:?}", merge_args.value_of("right"));
        }
        _ => ()
    }
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
            None,
        );
        let item_1a = item_1.clone();
        let item_2 =
            Item::build_test_item(ItemType::Part, ItemID(String::from("3039")), None, None);
        let item_2a = item_2.clone();
        let item_3 = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3001")),
            Some(Color(5)),
            None,
        );
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
        assert_eq!(hm.get(&key_1), Some(&item_1a));
        assert_eq!(hm.get(&key_2), Some(&item_2a));
    }

    #[test]
    fn test_increment_item_with_righthand_min_qty() {
        let mut left_item = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3039")),
            Some(Color(5)),
            Some(MinQty(20)),
        );
        let right_item = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3039")),
            Some(Color(5)),
            Some(MinQty(10)),
        );

        increment_item(&mut left_item, &right_item);
        assert_eq!(left_item.min_qty.unwrap().0, 30);
    }

    #[test]
    fn test_increment_item_with_no_righthand_min_qty() {
        let mut left_item = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3039")),
            Some(Color(5)),
            Some(MinQty(20)),
        );
        let right_item = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3039")),
            Some(Color(5)),
            None,
        );

        increment_item(&mut left_item, &right_item);
        assert_eq!(left_item.min_qty.unwrap().0, 21);
    }

    #[test]
    fn test_increment_item_with_no_min_qty() {
        let mut left_item = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3039")),
            Some(Color(5)),
            None,
        );
        let right_item = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3039")),
            Some(Color(5)),
            None,
        );

        increment_item(&mut left_item, &right_item);
        assert_eq!(left_item.min_qty.unwrap().0, 2);
    }
}
