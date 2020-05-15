use bricktools::inventory::{SerdeInventory, Inventory};
use bricktools::xml_to_string;

use quick_xml::de::from_str;

use std::path::PathBuf;

fn load_resource_directory() -> PathBuf {
    let mut resource_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    resource_dir.push("resources/test");
    resource_dir
}

fn get_resource_path(resource_name: &str) -> PathBuf {
    let mut resource_path = load_resource_directory();
    resource_path.push(resource_name);
    resource_path
}

pub fn resource_name_to_inventory(resource_name: &str) -> Inventory {
    let resource_path = get_resource_path(resource_name);
    let resource_str = xml_to_string(&resource_path).unwrap();
    Inventory::from(from_str::<SerdeInventory>(&resource_str).unwrap())
}
