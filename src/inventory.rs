//! A Bricklink Inventory
//!
//! These types are all based on the Bricklink
//! XML schema as described here: https://www.bricklink.com/help.asp?helpID=207

use serde::Deserialize;
/// The top level inventory that will hold a vector of Items
#[derive(Debug, Deserialize)]
pub struct Inventory {
    #[serde(rename = "ITEM")]
    pub items: Vec<Item>
}

/// A single Lego Item
#[derive(Debug, Deserialize)]
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
    pub wanted_list_id: Option<WantedListID>
}


/// The type of the Lego Item
#[derive(Debug, Deserialize)]
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
    UnsortedLot
}

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
            unsupported =>  Err(format!("{} is not a supported ItemType!", unsupported))
        }
    }
}

/// The canonical Lego catalog item number
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ItemID(String);

impl std::convert::TryFrom<String> for ItemID {
    type Error = String;
    fn try_from(input_str: String) -> Result<Self, Self::Error> {
        Ok(Self(input_str))
    }
}

/// Color ID according to the Bricklink color catalog 
/// https://www.bricklink.com/catalogColors.asp
#[derive(Debug, Deserialize)]
#[serde(try_from = "i8")]
pub struct Color(i8);

impl std::convert::TryFrom<i8> for Color {
    type Error = String;
    fn try_from(input_i8: i8) -> Result<Self, Self::Error> {
        Ok(Self(input_i8))
    }
}

/// Maximum Desired Price
#[derive(Debug, Deserialize)]
#[serde(try_from = "f32")]
pub struct MaxPrice(f32);

impl std::convert::TryFrom<f32> for MaxPrice {
    type Error = String;
    fn try_from(input_f32: f32) -> Result<Self, Self::Error> {
        Ok(Self(input_f32))
    }
}

/// Minimum desired quantity
#[derive(Debug, Deserialize)]
#[serde(try_from = "i32")]
pub struct MinQty(i32);

impl std::convert::TryFrom<i32> for MinQty {
    type Error = String;
    fn try_from(input_i32: i32) -> Result<Self, Self::Error> {
        Ok(Self(input_i32))
    }
}

/// Quantity of the item you already have
#[derive(Debug, Deserialize)]
#[serde(try_from = "i32")]
pub struct QtyFilled(i32);

impl std::convert::TryFrom<i32> for QtyFilled {
    type Error = String;
    fn try_from(input_i32: i32) -> Result<Self, Self::Error> {
        Ok(Self(input_i32))
    }
}

/// Item condition
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub enum Condition {
    New, 
    Used, 
    Complete, 
    Incomplete, 
    Sealed
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
            unsupported =>  Err(format!("{} is not a supported Condition!", unsupported))
        }
    }
}

/// Notes on the item
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Remarks(String);

impl std::convert::TryFrom<String> for Remarks {
    type Error = String;
    fn try_from(input_str: String) -> Result<Self, Self::Error> {
        Ok(Self(input_str))
    }
}

/// Be notified when these items are listed for sale
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub enum Notify {
	Y,
	N
}

impl std::convert::TryFrom<String> for Notify {
    type Error = String;
    fn try_from(notify_str: String) -> Result<Self, Self::Error> {
        match notify_str.as_str() {
            "Y" => return Ok(Self::Y),
            "N" => return Ok(Self::N),
            unsupported =>  Err(format!("{} is not a supported Notify!", unsupported))
        }
    }
}

/// Show in items for sale queries? 
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub enum WantedShow {
	Y, 
	N
}

impl std::convert::TryFrom<String> for WantedShow {
    type Error = String;
    fn try_from(wantedshow_str: String) -> Result<Self, Self::Error> {
        match wantedshow_str.as_str() {
            "Y" => return Ok(Self::Y),
            "N" => return Ok(Self::N),
            unsupported =>  Err(format!("{} is not a supported Notify!", unsupported))
        }
    }
}

/// ID of wanted list
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct WantedListID(String);

impl std::convert::TryFrom<String> for WantedListID {
    type Error = String;
    fn try_from(input_str: String) -> Result<Self, Self::Error> {
        Ok(Self(input_str))
    }
}
