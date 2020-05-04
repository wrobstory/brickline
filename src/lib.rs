mod inventory;

use crate::inventory::Inventory;

use std::io::{Read, Error as IOError};
use std::fs::File;
use std::path::PathBuf;

use quick_xml::de::{from_str, DeError};

fn xml_to_string(file_path: &PathBuf) -> Result<String, IOError> {
	let mut file = File::open(file_path)?;
	let mut xml_string = String::new();
	file.read_to_string(&mut xml_string)?;
	Ok(xml_string)
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
    	println!("{:?}", bricklink_inventory.items.first());
    }

}
