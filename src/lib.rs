mod inventory;

use crate::inventory::{
    Color, Condition, Inventory, Item, ItemID, ItemType, MaxPrice, MinQty, Notify, QtyFilled,
    Remarks,
};

use std::fs::File;
use std::io::{Error as IOError, Read};
use std::path::PathBuf;

use quick_xml::de::{from_str, DeError};

fn xml_to_string(file_path: &PathBuf) -> Result<String, IOError> {
    let mut file = File::open(file_path)?;
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string)?;
    Ok(xml_string)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ItemColorHashKey {
	item_id: ItemID,
	color: Color
}

#[cfg(test)]
mod tests {

    use super::*;

    fn load_resource_directory() -> PathBuf {
        let mut resource_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        resource_dir.push("resources/test");
        resource_dir
    }

    fn get_bricklink_example_path() -> PathBuf {
        let mut resource_path = load_resource_directory();
        resource_path.push("bricklink_example.xml");
        resource_path
    }

    #[test]
    fn test_xml_to_string_happy_path() {
        let bricklink_example = xml_to_string(&get_bricklink_example_path()).unwrap();
        assert!(bricklink_example.starts_with("<INVENTORY>"));
        assert!(bricklink_example.ends_with("</INVENTORY>"));
    }

    #[test]
    fn test_xml_to_inventory() {
        let bricklink_example_str = xml_to_string(&get_bricklink_example_path()).unwrap();
        let bricklink_inventory: Inventory = from_str(&bricklink_example_str).unwrap();

        let item_1 = Item {
            item_type: ItemType::Part,
            item_id: ItemID(String::from("3622")),
            color: Some(Color(11)),
            max_price: None,
            min_qty: None,
            qty_filled: Some(QtyFilled(4)),
            condition: None,
            remarks: None,
            notify: None,
            wanted_show: None,
            wanted_list_id: None,
        };
        let item_2 = Item {
            item_type: ItemType::Part,
            item_id: ItemID(String::from("3039")),
            color: None,
            max_price: None,
            min_qty: None,
            qty_filled: None,
            condition: None,
            remarks: None,
            notify: None,
            wanted_show: None,
            wanted_list_id: None,
        };
        let item_3 = Item {
            item_type: ItemType::Part,
            item_id: ItemID(String::from("3001")),
            color: Some(Color(5)),
            max_price: Some(MaxPrice(1.00)),
            min_qty: Some(MinQty(100)),
            qty_filled: None,
            condition: Some(Condition::New),
            remarks: Some(Remarks(String::from("for MOC AB154A"))),
            notify: Some(Notify::N),
            wanted_show: None,
            wanted_list_id: None,
        };
        let items = vec![item_1, item_2, item_3];
        let expected_inventory = Inventory { items: items };
        assert_eq!(bricklink_inventory, expected_inventory);
    }
}
