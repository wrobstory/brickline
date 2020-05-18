extern crate brickline;

use std::convert::TryFrom;

use brickline::wanted::{
    Color, Condition, WantedList, Item, ItemID, ItemType, MaxPrice, MinQty, Notify, QtyFilled,
    Remarks,
};

mod common;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_xml_to_wanted_list() {
        let bricklink_wanted_list: WantedList =
            common::resource_name_to_wanted_list("bricklink_example.xml");

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
        let expected_wanted_list = WantedList { items: items };
        assert_eq!(bricklink_wanted_list, expected_wanted_list);
    }

    #[test]
    fn test_wanted_list_to_string_1() {
        let item_1 = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3622")),
            Some(Color(11)),
            Some(MinQty(4)),
        );
        let items = vec![item_1];
        let wanted_list = WantedList { items: items };
        let stringified = String::try_from(wanted_list).unwrap();
        let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                <INVENTORY>\
                <ITEM>\
                <ITEMTYPE>P</ITEMTYPE>\
                <ITEMID>3622</ITEMID>\
                <COLOR>11</COLOR>\
                <MINQTY>4</MINQTY>\
                </ITEM>\
            </INVENTORY>\
            ";

        assert_eq!(String::from(expected), stringified);
    }

    #[test]
    fn test_wanted_list_to_string_2() {
        let item_1 = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3622")),
            Some(Color(11)),
            Some(MinQty(4)),
        );
        let item_2 = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3623")),
            Some(Color(11)),
            Some(MinQty(4)),
        );
        let item_3 = Item::build_test_item(
            ItemType::Part,
            ItemID(String::from("3624")),
            Some(Color(11)),
            Some(MinQty(4)),
        );
        let items = vec![item_1, item_2, item_3];
        let wanted_list = WantedList { items: items };
        let stringified = String::try_from(wanted_list).unwrap();
        let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                <INVENTORY>\
                <ITEM>\
                <ITEMTYPE>P</ITEMTYPE>\
                <ITEMID>3622</ITEMID>\
                <COLOR>11</COLOR>\
                <MINQTY>4</MINQTY>\
                </ITEM>\
                <ITEM>\
                <ITEMTYPE>P</ITEMTYPE>\
                <ITEMID>3623</ITEMID>\
                <COLOR>11</COLOR>\
                <MINQTY>4</MINQTY>\
                </ITEM>\
                <ITEM>\
                <ITEMTYPE>P</ITEMTYPE>\
                <ITEMID>3624</ITEMID>\
                <COLOR>11</COLOR>\
                <MINQTY>4</MINQTY>\
                </ITEM>\
            </INVENTORY>\
            ";

        assert_eq!(String::from(expected), stringified);
    }

    #[test]
    fn test_roundtrips() {
        for resource_name in vec![
            "bricklink_example.xml",
            "test_wanted_list_1.xml",
            "test_wanted_list_2.xml",
            "test_wanted_list_3.xml",
        ]
        .iter()
        {
            let wanted_list = common::resource_name_to_wanted_list(resource_name);
            let stringified = String::try_from(wanted_list).unwrap();
            let expected_string = common::resource_name_to_string(resource_name);
            assert_eq!(expected_string, stringified);
        }
    }
}
