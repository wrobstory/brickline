pub mod wanted;

use crate::wanted::{Color, WantedList, Item, ItemID, MinQty, SerdeWantedList};

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::error;
use std::fs::File;
use std::io::{Error as IOError, ErrorKind, Read, Write};
use std::path::PathBuf;

use clap::ArgMatches;
use quick_xml::de::from_str;

/// The primary key of an WantedList Item
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemColorHashKey<'a> {
    item_id: &'a ItemID,
    color: &'a Option<Color>,
}

/// Get user input from stdout
///
/// # Arguments
///
/// * `message`: What message do you want to prompt the user with?
///
fn stdout_input(message: &str) -> Result<String, std::io::Error> {
    print!("{}", message);
    std::io::stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

/// Write a file. If the file already exists, prompt the user to ask
/// if they want to overwrite it.
///
/// # Arguments
///
/// * `file_path`: Path to file to write
/// * `content`: File content to write
///
fn write_file_with_overwrite_prompt(
    file_path: &PathBuf,
    content: &String,
) -> Result<(), std::io::Error> {
    if file_path.exists() {
        let msg = format!(
            "The file {} already exists. Do you want to overwrite this file? ",
            file_path.to_str().unwrap()
        );
        let overwrite = stdout_input(&msg)?;
        let lower = overwrite.to_lowercase();
        let trimmed = lower.trim();
        if trimmed != "y" && trimmed != "yes" {
            println!("Exited without writing file");
            std::process::exit(0x0100);
        }
    }

    let mut file = File::create(file_path)?;
    println!(
        "Writing joined wanted list to {}",
        file_path.to_str().unwrap()
    );
    file.write_all(content.as_bytes())?;
    Ok(())
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
/// use brickline::xml_to_string;
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

/// Given a path to a file, read the file and deserialize it to an WantedList
///
/// # Arguments
///
/// * `file_path`: String path to file
///
/// Example
///
/// ```no_run
/// use brickline::file_to_inventory;
///
/// let inventory = file_to_inventory("/path/to/wanted_list.xml");
pub fn file_to_inventory(file_path: &str) -> Result<WantedList, IOError> {
    let resource_path = PathBuf::from(file_path);
    let resource_str = xml_to_string(&resource_path)?;
    match from_str::<SerdeWantedList>(&resource_str) {
        Ok(serde_inventory) => Ok(WantedList::from(serde_inventory)),
        Err(e) => Err(IOError::new(ErrorKind::InvalidInput, e)),
    }
}

/// Given an WantedList, build a HashMap of each WantedList Item where
/// the hash key is the ItemID and Color combination for the Item.
/// Note: we explicitly .clone the Item for this map, as we're going to
/// use it as the base case for our joined list.
///
/// # Arguments
///
/// * `inventory`: Bricklink inventory as deserialized from XML
///
/// Example
///
/// ```no_run
/// use brickline::{xml_to_string, build_item_color_hashmap};
/// use brickline::wanted::{WantedList, SerdeWantedList};
/// use quick_xml::de::from_str;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("/home/user/path/to/file.xml");
/// let xml_string = xml_to_string(&path).unwrap();
/// let inventory = WantedList::from(from_str::<SerdeWantedList>(&xml_string).unwrap());
/// let hm = build_item_color_hashmap(&inventory);
/// ```
pub fn build_item_color_hashmap(inventory: &WantedList) -> BTreeMap<ItemColorHashKey, Item> {
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
/// use brickline::increment_item;
/// use brickline::wanted::Item;
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

/// Given two Inventories, join the right inventory into the left one.
/// Here's how the join happens:
/// 1. Build hash table from left inventory
/// 2. Iterate through right inventory and probe table for ItemId/Color keys
/// 3. If a key is found, add the MinQty of the right inventory to the left.
///    NOTE: The metadata from the *left* inventory is retained. There is no
///    other metadata joining other than MinQty.
/// 4. If no key is found, add the Item from the right inventory to the hash table
/// 5. Convert the .values() of the hash table into .items of a new WantedList
///
/// # Arguments
///
/// * `left_inventory`: WantedList to be joined into
/// * `right_inventory`: WantedList to join into left inventory
///
/// Example
///
/// ```
/// use brickline::join_inventories;
/// use brickline::wanted::{WantedList, Item, ItemID, ItemType, Color, MinQty};
///
/// let item = Item::build_test_item(
///       ItemType::Part,
///       ItemID(String::from("3039")),
///       Some(Color(5)),
///       Some(MinQty(20)),
/// );
/// let item_1 = item.clone();
///
/// let left_inventory = WantedList { items: vec![item] };
/// let right_inventory = WantedList { items: vec![item_1] };
///
/// let joined_inventory = join_inventories(&left_inventory, &right_inventory);
/// ```
pub fn join_inventories(left_inventory: &WantedList, right_inventory: &WantedList) -> WantedList {
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
    WantedList {
        items: left_inv_map.values().cloned().collect(),
    }
}

/// Given the arguments for the `join` command, join the two wanted lists,
/// then write the result to the provided output path.
///
/// # Arguments
///
/// * `join_args`: Arguments to the join command
///
pub fn join(join_args: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let left_path = join_args.value_of("left").ok_or(IOError::new(
        ErrorKind::InvalidInput,
        "Empty left inventory path",
    ))?;
    let right_path = join_args.value_of("right").ok_or(IOError::new(
        ErrorKind::InvalidInput,
        "Empty right inventory path",
    ))?;
    let left_inventory = file_to_inventory(left_path)?;
    let right_inventory = file_to_inventory(right_path)?;
    println!("Left Bricklink Wanted List: {}", left_path);
    println!("Right Bricklink Wanted List: {}", right_path);
    println!("Merging wanted lists...");
    let joined_inventory = join_inventories(&left_inventory, &right_inventory);
    let xml_string = String::try_from(joined_inventory)?;

    let out_path_str = join_args
        .value_of("output")
        .ok_or(IOError::new(ErrorKind::InvalidInput, "Empty output path"))?;
    let out_path = PathBuf::from(out_path_str);
    write_file_with_overwrite_prompt(&out_path, &xml_string)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::wanted::ItemType;

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
        let inventory = WantedList {
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
