extern crate bricktools;

use bricktools::inventory::{ItemID, MinQty, Remarks};

mod common;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge_inventories_1() {
        let inventory_1 = common::resource_name_to_inventory("test_inventory_1.xml");
        let inventory_2 = common::resource_name_to_inventory("test_inventory_2.xml");

        let merged_inventory_1 = bricktools::merge_inventories(&inventory_1, &inventory_2);
        let merged_inventory_2 = bricktools::merge_inventories(&inventory_2, &inventory_1);

        // These end up being ordered by ItemID
        let expected_qty = vec![
            (ItemID(String::from("3000")), Some(MinQty(4))),
            (ItemID(String::from("3001")), Some(MinQty(200))),
            (ItemID(String::from("3622")), Some(MinQty(14))),
            (ItemID(String::from("3623")), None),
        ];
        for (i, expected) in expected_qty.iter().enumerate() {
            let item = &merged_inventory_1.items[i];
            assert_eq!(expected.0, item.item_id);
            assert_eq!(expected.1, item.min_qty)
        }

        // The first merge should retain the remarks of inventory_1
        assert_eq!(
            merged_inventory_1.items[1].remarks,
            Some(Remarks("Testing".to_string()))
        );
        // The second merge should use inventory_2, so no remarks
        assert_eq!(merged_inventory_2.items[1].remarks, None);
    }

    #[test]
    fn test_merge_inventories_2() {
        let inventory_1 = common::resource_name_to_inventory("test_inventory_1.xml");
        let inventory_2 = common::resource_name_to_inventory("bricklink_example.xml");

        let merged_inventory_1 = bricktools::merge_inventories(&inventory_1, &inventory_2);
        let merged_inventory_2 = bricktools::merge_inventories(&inventory_2, &inventory_1);

        // These end up being ordered by ItemID
        let expected_qty = vec![
            (ItemID(String::from("3001")), Some(MinQty(200))),
            (ItemID(String::from("3039")), None),
            (ItemID(String::from("3622")), Some(MinQty(5))),
            (ItemID(String::from("3623")), None),
        ];
        for (i, expected) in expected_qty.iter().enumerate() {
            let item = &merged_inventory_1.items[i];
            assert_eq!(expected.0, item.item_id);
            assert_eq!(expected.1, item.min_qty)
        }

        // The first merge should retain the remarks of inventory_1
        assert_eq!(
            merged_inventory_1.items[0].remarks,
            Some(Remarks("Testing".to_string()))
        );
        // The second merge should retain the remarks of bricklink_example
        assert_eq!(
            merged_inventory_2.items[0].remarks,
            Some(Remarks("for MOC AB154A".to_string()))
        );
    }
}
