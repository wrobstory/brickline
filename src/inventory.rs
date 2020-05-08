//! A Bricklink Inventory
//!
//! These types are all based on the Bricklink
//! XML schema as described here: https://www.bricklink.com/help.asp?helpID=207
//!
//! All of the impl std::convert::TryFrom<N> for T logic is a workaround for
//! deserialization of XML to enum.

use serde::Deserialize;
/// The top level inventory that will hold a vector of Items
#[derive(Debug, Deserialize, PartialEq)]
pub struct Inventory {
    #[serde(rename = "ITEM")]
    pub items: Vec<Item>,
}

/// A single Lego Item
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Item {
    #[serde(rename = "ITEMTYPE")]
    pub item_type: ItemType,
    #[serde(rename = "ITEMID")]
    pub item_id: ItemID,
    #[serde(rename = "COLOR")]
    pub color: Option<Color>,
    #[serde(rename = "MAXPRICE")]
    pub max_price: Option<MaxPrice>,
    #[serde(rename = "MINQTY")]
    pub min_qty: Option<MinQty>,
    #[serde(rename = "QTYFILLED")]
    pub qty_filled: Option<QtyFilled>,
    #[serde(rename = "CONDITION")]
    pub condition: Option<Condition>,
    #[serde(rename = "REMARKS")]
    pub remarks: Option<Remarks>,
    #[serde(rename = "NOTIFY")]
    pub notify: Option<Notify>,
    #[serde(rename = "WANTEDSHOW")]
    pub wanted_show: Option<WantedShow>,
    #[serde(rename = "WANTEDLISTID")]
    pub wanted_list_id: Option<WantedListID>,
}

impl Item {
    /// Build a test Item with item_type, item_id, color, and all other fields set to
    /// None. Only used as a test data generator.
    ///
    /// # Arguments
    ///
    /// * `item_type` - ItemType
    /// * `item_id` - ItemID
    /// * `color` - Color
    ///
    /// # Example
    ///
    /// ```
    /// use bricktools::inventory::{Item, ItemType, ItemID, Color};
    ///
    /// let test_item = Item::build_test_item(
    ///     ItemType::Part,
    ///     ItemID(String::from("3622")),
    ///     Some(Color(11)),
    /// );
    /// ```
    pub fn build_test_item(item_type: ItemType, item_id: ItemID, color: Option<Color>) -> Item {
        Item {
            item_type,
            item_id,
            color,
            max_price: None,
            min_qty: None,
            qty_filled: None,
            condition: None,
            remarks: None,
            notify: None,
            wanted_show: None,
            wanted_list_id: None,
        }
    }
}

/// The type of the Lego Item
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "String")]
pub enum ItemType {
    Set,
    Part,
    Minifig,
    Book,
    Gear,
    Catalog,
    Instruction,
    OriginalBox,
    UnsortedLot,
}

/// Workaround for deserialization from XML to enum
impl std::convert::TryFrom<String> for ItemType {
    type Error = String;
    fn try_from(itemtype_str: String) -> Result<Self, Self::Error> {
        match itemtype_str.as_str() {
            "S" => return Ok(Self::Set),
            "P" => return Ok(Self::Part),
            "M" => return Ok(Self::Minifig),
            "B" => return Ok(Self::Book),
            "G" => return Ok(Self::Gear),
            "C" => return Ok(Self::Catalog),
            "I" => return Ok(Self::Instruction),
            "O" => return Ok(Self::OriginalBox),
            "U" => return Ok(Self::UnsortedLot),
            unsupported => Err(format!("{} is not a supported ItemType!", unsupported)),
        }
    }
}

/// The canonical Lego catalog item number
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct ItemID(pub String);

impl std::convert::TryFrom<String> for ItemID {
    type Error = String;
    fn try_from(input_str: String) -> Result<Self, Self::Error> {
        Ok(Self(input_str))
    }
}

/// Color ID according to the Bricklink color catalog
/// https://www.bricklink.com/catalogColors.asp
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[serde(try_from = "i8")]
pub struct Color(pub i8);

impl std::convert::TryFrom<i8> for Color {
    type Error = String;
    fn try_from(input_i8: i8) -> Result<Self, Self::Error> {
        Ok(Self(input_i8))
    }
}

/// Maximum Desired Price
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "f32")]
pub struct MaxPrice(pub f32);

impl std::convert::TryFrom<f32> for MaxPrice {
    type Error = String;
    fn try_from(input_f32: f32) -> Result<Self, Self::Error> {
        Ok(Self(input_f32))
    }
}

/// Minimum desired quantity
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "i32")]
pub struct MinQty(pub i32);

impl std::convert::TryFrom<i32> for MinQty {
    type Error = String;
    fn try_from(input_i32: i32) -> Result<Self, Self::Error> {
        Ok(Self(input_i32))
    }
}

/// Quantity of the item you already have
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "i32")]
pub struct QtyFilled(pub i32);

impl std::convert::TryFrom<i32> for QtyFilled {
    type Error = String;
    fn try_from(input_i32: i32) -> Result<Self, Self::Error> {
        Ok(Self(input_i32))
    }
}

/// Item condition
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "String")]
pub enum Condition {
    New,
    Used,
    Complete,
    Incomplete,
    Sealed,
}

impl std::convert::TryFrom<String> for Condition {
    type Error = String;
    fn try_from(condition_str: String) -> Result<Self, Self::Error> {
        match condition_str.as_str() {
            "N" => return Ok(Self::New),
            "U" => return Ok(Self::Used),
            "C" => return Ok(Self::Complete),
            "I" => return Ok(Self::Incomplete),
            "S" => return Ok(Self::Sealed),
            unsupported => Err(format!("{} is not a supported Condition!", unsupported)),
        }
    }
}

/// Notes on the item
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "String")]
pub struct Remarks(pub String);

impl std::convert::TryFrom<String> for Remarks {
    type Error = String;
    fn try_from(input_str: String) -> Result<Self, Self::Error> {
        Ok(Self(input_str))
    }
}

/// Be notified when these items are listed for sale
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "String")]
pub enum Notify {
    Y,
    N,
}

impl std::convert::TryFrom<String> for Notify {
    type Error = String;
    fn try_from(notify_str: String) -> Result<Self, Self::Error> {
        match notify_str.as_str() {
            "Y" => return Ok(Self::Y),
            "N" => return Ok(Self::N),
            unsupported => Err(format!("{} is not a supported Notify!", unsupported)),
        }
    }
}

/// Show in items for sale queries?
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "String")]
pub enum WantedShow {
    Y,
    N,
}

impl std::convert::TryFrom<String> for WantedShow {
    type Error = String;
    fn try_from(wantedshow_str: String) -> Result<Self, Self::Error> {
        match wantedshow_str.as_str() {
            "Y" => return Ok(Self::Y),
            "N" => return Ok(Self::N),
            unsupported => Err(format!("{} is not a supported Notify!", unsupported)),
        }
    }
}

/// ID of wanted list
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(try_from = "String")]
pub struct WantedListID(String);

impl std::convert::TryFrom<String> for WantedListID {
    type Error = String;
    fn try_from(input_str: String) -> Result<Self, Self::Error> {
        Ok(Self(input_str))
    }
}
