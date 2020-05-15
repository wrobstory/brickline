//! A Bricklink Inventory
//!
//! These types are all based on the Bricklink
//! XML schema as described here: https://www.bricklink.com/help.asp?helpID=207
//!
//! All of the impl std::convert::TryFrom<N> for T logic is a workaround for
//! deserialization of XML to enum.

use serde::{Deserialize, Serialize, Serializer};
/// The top level inventory that will hold a vector of Items
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SerdeInventory {
    #[serde(rename = "ITEM")]
    pub items: Vec<SerdeItem>,
}

#[derive(Debug, PartialEq)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl std::convert::From<SerdeInventory> for Inventory {
    fn from(serde_inventory: SerdeInventory) -> Inventory {
        Inventory {
            items: serde_inventory.items.into_iter().map(|i| Item::from(i)).collect()
        }
    }
}

impl std::convert::From<Inventory> for SerdeInventory {
    fn from(inventory: Inventory) -> SerdeInventory {
        SerdeInventory {
            items: inventory.items.into_iter().map(|i| SerdeItem::from(i)).collect()
        }
    }
}

/// A single Lego Item
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SerdeItem {
    #[serde(rename = "ITEMTYPE")]
    pub item_type: String,
    #[serde(rename = "ITEMID")]
    pub item_id: String,
    #[serde(rename = "COLOR")]
    pub color: Option<i8>,
    #[serde(rename = "MAXPRICE")]
    pub max_price: Option<f32>,
    #[serde(rename = "MINQTY")]
    pub min_qty: Option<i32>,
    #[serde(rename = "QTYFILLED")]
    pub qty_filled: Option<i32>,
    #[serde(rename = "CONDITION")]
    pub condition: Option<String>,
    #[serde(rename = "REMARKS")]
    pub remarks: Option<String>,
    #[serde(rename = "NOTIFY")]
    pub notify: Option<String>,
    #[serde(rename = "WANTEDSHOW")]
    pub wanted_show: Option<String>,
    #[serde(rename = "WANTEDLISTID")]
    pub wanted_list_id: Option<String>,
}

/// A single Lego Item
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub item_type: ItemType,
    pub item_id: ItemID,
    pub color: Option<Color>,
    pub max_price: Option<MaxPrice>,
    pub min_qty: Option<MinQty>,
    pub qty_filled: Option<QtyFilled>,
    pub condition: Option<Condition>,
    pub remarks: Option<Remarks>,
    pub notify: Option<Notify>,
    pub wanted_show: Option<WantedShow>,
    pub wanted_list_id: Option<WantedListID>,
}

impl std::convert::From<SerdeItem> for Item {
    fn from(serde_item: SerdeItem) -> Item {
        Item {
            item_type: ItemType::try_from(serde_item.item_type),
            item_id: ItemID::from(serde_item.item_id),
            color: serde_item.color.map(|c| Color::from(c)),
            max_price: serde_item.max_price.map(|m| MaxPrice::from(m)),
            min_qty: serde_item.min_qty.map(|m| MinQty::from(m)),
            qty_filled: serde_item.qty_filled.map(|q| QtyFilled::from(q)),
            condition: serde_item.condition.map(|c| Condition::try_from(c)),
            remarks: serde_item.remarks.map(|r| Remarks::from(r)),
            notify: Notify::try_from(serde_item.notify),
            wanted_show: WantedShow::try_from(serde_item.wanted_show),
            wanted_list_id: serde_item.wanted_list_id.map(|w| WantedListID::from(w))
        }
    }
}

impl std::convert::From<Item> for SerdeItem {
    fn from(item: Item) -> SerdeItem {
        SerdeItem {
            item_type: item.item_type,
            item_id: String::from(item.item_id),
            color: item.color.map(|c| i8::from(c)),
            max_price: item.max_price.map(|m| f32::from(m)),
            min_qty: item.min_qty.map(|m| i32::from(m)),
            qty_filled: item.qty_filled.map(|q| i32::from(q)),
            condition: item.condition,
            remarks: item.remarks.map(|r| String::from(r)),
            notify: item.notify,
            wanted_show: item.wanted_show,
            wanted_list_id: item.wanted_list_id.map(|w| String::from(w))
        }
    }
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
    ///     None
    /// );
    /// ```
    pub fn build_test_item(
        item_type: ItemType,
        item_id: ItemID,
        color: Option<Color>,
        min_qty: Option<MinQty>,
    ) -> Item {
        Item {
            item_type,
            item_id: item_id.into(),
            color: color.map(|c| c.into()),
            min_qty: min_qty.map(|m| m.into()),
            max_price: None,
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize)]
pub struct ItemID(pub String);

impl std::convert::From<String> for ItemID {
    fn from(input_str: String) -> ItemID {
        Self(input_str)
    }
}

impl std::convert::From<ItemID> for String {
    fn from(item_id: ItemID) -> String {
        item_id.0
    }
}

/// Color ID according to the Bricklink color catalog
/// https://www.bricklink.com/catalogColors.asp
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Color(pub i8);

impl std::convert::From<i8> for Color {
    fn from(input_i8: i8) -> Color {
        Self(input_i8)
    }
}

impl std::convert::From<Color> for i8 {
    fn from(color: Color) -> i8 {
        color.0
    }
}

/// Maximum Desired Price
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(try_from = "f32")]
pub struct MaxPrice(pub f32);

impl std::convert::From<f32> for MaxPrice {
    fn from(input_f32: f32) -> MaxPrice {
        Self(input_f32)
    }
}

impl std::convert::From<MaxPrice> for f32 {
    fn from(max_price: MaxPrice) -> f32 {
        max_price.0
    }
}

/// Minimum desired quantity
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinQty(pub i32);

impl std::convert::From<i32> for MinQty {
    fn from(input_i32: i32) -> MinQty {
        Self(input_i32)
    }
}

impl std::convert::From<MinQty> for i32 {
    fn from(min_qty: MinQty) -> i32{
        min_qty.0
    }
}

/// Quantity of the item you already have
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QtyFilled(pub i32);

impl std::convert::From<i32> for QtyFilled {
    fn from(input_i32: i32) -> QtyFilled {
        Self(input_i32)
    }
}

impl std::convert::From<QtyFilled> for i32 {
    fn from(qty_filled: QtyFilled) -> i32 {
        qty_filled.0
    }
}

/// Item condition
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Remarks(pub String);

impl std::convert::From<String> for Remarks {
    fn from(input_str: String) -> Remarks {
        Self(input_str)
    }
}

impl std::convert::From<Remarks> for String {
    fn from(remarks: Remarks) -> String {
        remarks.0
    }
}

/// Be notified when these items are listed for sale
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WantedListID(String);

impl std::convert::From<String> for WantedListID {
    fn from(input_str: String) -> WantedListID {
        Self(input_str)
    }
}

impl std::convert::From<WantedListID> for String {
    fn from(wanted_list_id: WantedListID) -> String {
        wanted_list_id.0
    }
}
