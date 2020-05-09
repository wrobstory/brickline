extern crate bricktools;

use bricktools::inventory::{
    Color, Condition, Inventory, Item, ItemID, ItemType, MaxPrice, MinQty, Notify, QtyFilled,
    Remarks,
};

mod common;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge_inventories_1() {
        let inventory_1 = common::resource_name_to_inventory("test_inventory_1.xml");
        let inventory_2 = common::resource_name_to_inventory("test_inventory_2.xml");

        let merged_inventory = bricktools::merge_inventories(&inventory_1, &inventory_2);

        // These end up being ordered by ItemID
        let expected_qty = vec![
            (ItemID(String::from("3000")), Some(MinQty(4))),
            (ItemID(String::from("3001")), Some(MinQty(200))),
            (ItemID(String::from("3622")), Some(MinQty(14))),
            (ItemID(String::from("3623")), None),
        ];
        for (i, expected) in expected_qty.iter().enumerate() {
            let item = &merged_inventory.items[i];
            assert_eq!(expected.0, item.item_id);
            assert_eq!(expected.1, item.min_qty)
        }
    }
}
