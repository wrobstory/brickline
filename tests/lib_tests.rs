extern crate brickline;

use brickline::wanted::{ItemID, MinQty, Remarks};

mod common;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_join_inventories_1() {
        let wanted_list_1 = common::resource_name_to_wanted_list("test_wanted_list_1.xml");
        let wanted_list_2 = common::resource_name_to_wanted_list("test_wanted_list_2.xml");

        let joined_wanted_list_1 = brickline::join_inventories(&wanted_list_1, &wanted_list_2);
        let joined_wanted_list_2 = brickline::join_inventories(&wanted_list_2, &wanted_list_1);

        // These end up being ordered by ItemID
        let expected_qty = vec![
            (ItemID(String::from("3000")), Some(MinQty(4))),
            (ItemID(String::from("3001")), Some(MinQty(200))),
            (ItemID(String::from("3622")), Some(MinQty(14))),
            (ItemID(String::from("3623")), None),
        ];
        for (i, expected) in expected_qty.iter().enumerate() {
            let item = &joined_wanted_list_1.items[i];
            assert_eq!(expected.0, item.item_id);
            assert_eq!(expected.1, item.min_qty)
        }

        // The first join should retain the remarks of wanted_list_1
        assert_eq!(
            joined_wanted_list_1.items[1].remarks,
            Some(Remarks("Testing".to_string()))
        );
        // The second join should use wanted_list_2, so no remarks
        assert_eq!(joined_wanted_list_2.items[1].remarks, None);
    }

    #[test]
    fn test_join_inventories_2() {
        let wanted_list_1 = common::resource_name_to_wanted_list("test_wanted_list_1.xml");
        let wanted_list_2 = common::resource_name_to_wanted_list("bricklink_example.xml");

        let joined_wanted_list_1 = brickline::join_inventories(&wanted_list_1, &wanted_list_2);
        let joined_wanted_list_2 = brickline::join_inventories(&wanted_list_2, &wanted_list_1);

        // These end up being ordered by ItemID
        let expected_qty = vec![
            (ItemID(String::from("3001")), Some(MinQty(200))),
            (ItemID(String::from("3039")), None),
            (ItemID(String::from("3622")), Some(MinQty(5))),
            (ItemID(String::from("3623")), None),
        ];
        for (i, expected) in expected_qty.iter().enumerate() {
            let item = &joined_wanted_list_1.items[i];
            assert_eq!(expected.0, item.item_id);
            assert_eq!(expected.1, item.min_qty)
        }

        // The first join should retain the remarks of wanted_list_1
        assert_eq!(
            joined_wanted_list_1.items[0].remarks,
            Some(Remarks("Testing".to_string()))
        );
        // The second join should retain the remarks of bricklink_example
        assert_eq!(
            joined_wanted_list_2.items[0].remarks,
            Some(Remarks("for MOC AB154A".to_string()))
        );
    }
}
