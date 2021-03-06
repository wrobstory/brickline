//! A Bricklink Wanted List
//!
//! These types are all based on the Bricklink
//! XML schema as described here: https://www.bricklink.com/help.asp?helpID=207
//!
//! So what's going on here? The quick_xml library has relatively limited support
//! for complex types, and Rust doesn't have good support for a serializable decimel
//! type. So between those two things we've ended up with SerdeIn and SerdeItem,
//! top level structs that only use primitive types. We then iterate over the entire
//! list of items and do a bunch of From/Into transformations to go from our primitive
//! types to more complex ones. It's a bummer, but I don't expect to ever have Bricklink
//! wanted lists longer than O(thousands) of Items, so I'm willing to take perf hit
//! to do the full scan for deserialization/serialization.
use quick_xml::se::to_string;
use quick_xml::DeError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The serde wanted_list of SerdeItems
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename(serialize = "INVENTORY"))]
pub struct SerdeWantedList {
    #[serde(rename = "ITEM")]
    pub items: Vec<SerdeItem>,
}

impl SerdeWantedList {
    /// Dirty fix for a serialization issue with the quick_xml library.
    /// When we try to serialize a Vec<SerdeItem>, we end up with
    /// <ITEM><ITEM>...</ITEM></ITEM> at the beginning and end of the
    /// vectors. So...we're going to straight up remove the redundant
    /// Items by replacing those ranges in the String.
    ///
    ///
    /// # Arguments
    ///
    /// * `serde_string`: Serialized String of a SerdeWantedList
    ///
    /// # Example
    ///
    /// ```
    /// use quick_xml::se::to_string;
    /// use brickline::wanted::{WantedList, SerdeWantedList, Item, ItemType,
    ///                         ItemID, Color};
    ///
    /// let test_item = Item::build_test_item(
    ///     ItemType::Part,
    ///     ItemID(String::from("3622")),
    ///     Some(Color(11)),
    ///     None
    /// );
    /// let wanted_list = WantedList { items: vec![test_item]};
    /// let serde_wanted_list = SerdeWantedList::from(wanted_list);
    /// let stringified = to_string(&serde_wanted_list).unwrap();
    /// let repaired = SerdeWantedList::amend_serialized_string(stringified);
    /// ```
    pub fn amend_serialized_string(mut serde_string: String) -> String {
        serde_string.replace_range(11..17, "");
        let end_bound_1 = serde_string.len() - 19;
        let end_bound_2 = serde_string.len() - 12;
        serde_string.replace_range(end_bound_1..end_bound_2, "");
        serde_string.insert_str(0, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        serde_string
    }
}

/// A Bricklink WantedList
#[derive(Debug, PartialEq)]
pub struct WantedList {
    pub items: Vec<Item>,
}

/// Serialize an WantedList to an XML String
impl std::convert::TryFrom<WantedList> for String {
    type Error = DeError;

    /// Given an WantedList, convert it to an XML string.
    /// This will go through the SerdeWantedList type as well as
    /// apply some of the ad-hoc fixes needed to make it a valid
    /// XML string.
    ///
    /// # Arguments
    ///
    /// * `wanted_list`: Bricklink WantedList
    /// ```
    fn try_from(wanted_list: WantedList) -> Result<Self, Self::Error> {
        let serde_wanted_list = SerdeWantedList::from(wanted_list);
        let stringified = to_string(&serde_wanted_list)?;
        Ok(SerdeWantedList::amend_serialized_string(stringified))
    }
}

#[derive(Debug, PartialEq)]
pub struct WantedListStatistics {
    pub total_items: i32,
    pub total_parts: i32,
    pub unique_item_color_count: i32,
    pub unique_color_count: i32,

    pub item_color_set: HashSet<OwnedItemColorHashKey>,
    pub color_set: HashSet<Color>,
}

impl WantedListStatistics {
    pub fn init() -> WantedListStatistics {
        WantedListStatistics {
            total_items: 0,
            total_parts: 0,
            unique_item_color_count: 0,
            unique_color_count: 0,
            item_color_set: HashSet::new(),
            color_set: HashSet::new(),
        }
    }
}

impl std::fmt::Display for WantedListStatistics {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "
Total Items: {}, 
Total Parts: {}, 
Unique Item/Color Count: {}, 
Unique Color Count: {}",
            self.total_items,
            self.total_parts,
            self.unique_item_color_count,
            self.unique_color_count
        )
    }
}

/// The primary key of an WantedList Item
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnedItemColorHashKey {
    item_id: ItemID,
    color: Option<Color>,
}

pub fn update_wanted_list_statistic(item: &Item, aggregate: &mut WantedListStatistics) -> () {
    aggregate.total_items += 1;

    match &item.min_qty {
        Some(min_qty) => aggregate.total_parts += min_qty.0,
        None => aggregate.total_parts += 1,
    };

    let ic_hk = OwnedItemColorHashKey {
        item_id: item.item_id.clone(),
        color: item.color.clone(),
    };

    if !aggregate.item_color_set.contains(&ic_hk) {
        aggregate.unique_item_color_count += 1;
        aggregate.item_color_set.insert(ic_hk);
    }

    item.color.as_ref().map(|color| {
        if !aggregate.color_set.contains(color) {
            aggregate.unique_color_count += 1;
            aggregate.color_set.insert(color.clone());
        }
    });
}

pub fn type_and_gen_statistics(
    serde_wanted_list: SerdeWantedList,
) -> (WantedList, WantedListStatistics) {
    let mut statistics = WantedListStatistics {
        total_items: 0,
        total_parts: 0,
        unique_item_color_count: 0,
        unique_color_count: 0,
        item_color_set: HashSet::new(),
        color_set: HashSet::new(),
    };

    let items = serde_wanted_list
        .items
        .into_iter()
        .map(|i| {
            let item = Item::from(i);
            update_wanted_list_statistic(&item, &mut statistics);
            item
        })
        .collect();

    (WantedList { items: items }, statistics)
}

// TODO: Unify the above and below
pub fn gen_statistics(wanted_list: &WantedList) -> WantedListStatistics {
    let mut statistics = WantedListStatistics::init();
    wanted_list
        .items
        .iter()
        .for_each(|item| update_wanted_list_statistic(item, &mut statistics));
    statistics
}

impl std::convert::From<WantedList> for SerdeWantedList {
    fn from(wanted_list: WantedList) -> SerdeWantedList {
        SerdeWantedList {
            items: wanted_list
                .items
                .into_iter()
                .map(|i| SerdeItem::from(i))
                .collect(),
        }
    }
}

impl std::convert::From<SerdeWantedList> for WantedList {
    fn from(serde_wanted_list: SerdeWantedList) -> WantedList {
        WantedList {
            items: serde_wanted_list
                .items
                .into_iter()
                .map(|i| Item::from(i))
                .collect(),
        }
    }
}

/// A serde representation of an Item
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename(serialize = "ITEM"))]
pub struct SerdeItem {
    #[serde(rename = "ITEMTYPE")]
    pub item_type: String,
    #[serde(rename = "ITEMID")]
    pub item_id: String,
    #[serde(rename = "COLOR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i8>,
    #[serde(rename = "MAXPRICE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<String>,
    #[serde(rename = "MINQTY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_qty: Option<i32>,
    #[serde(rename = "QTYFILLED")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_filled: Option<i32>,
    #[serde(rename = "CONDITION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "REMARKS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(rename = "NOTIFY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<String>,
    #[serde(rename = "WANTEDSHOW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wanted_show: Option<String>,
    #[serde(rename = "WANTEDLISTID")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
            item_type: ItemType::from(serde_item.item_type),
            item_id: ItemID::from(serde_item.item_id),
            color: serde_item.color.map(|c| Color::from(c)),
            max_price: serde_item.max_price.map(|m| MaxPrice::from(m)),
            min_qty: serde_item.min_qty.map(|m| MinQty::from(m)),
            qty_filled: serde_item.qty_filled.map(|q| QtyFilled::from(q)),
            condition: serde_item.condition.map(|c| Condition::from(c)),
            remarks: serde_item.remarks.map(|r| Remarks::from(r)),
            notify: serde_item.notify.map(|n| Notify::from(n)),
            wanted_show: serde_item.wanted_show.map(|w| WantedShow::from(w)),
            wanted_list_id: serde_item.wanted_list_id.map(|w| WantedListID::from(w)),
        }
    }
}

impl std::convert::From<Item> for SerdeItem {
    fn from(item: Item) -> SerdeItem {
        SerdeItem {
            item_type: String::from(item.item_type),
            item_id: String::from(item.item_id),
            color: item.color.map(|c| i8::from(c)),
            max_price: item.max_price.map(|m| String::from(m)),
            min_qty: item.min_qty.map(|m| i32::from(m)),
            qty_filled: item.qty_filled.map(|q| i32::from(q)),
            condition: item.condition.map(|c| String::from(c)),
            remarks: item.remarks.map(|r| String::from(r)),
            notify: item.notify.map(|n| String::from(n)),
            wanted_show: item.wanted_show.map(|ws| String::from(ws)),
            wanted_list_id: item.wanted_list_id.map(|w| String::from(w)),
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
    /// use brickline::wanted::{Item, ItemType, ItemID, Color};
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
#[derive(Clone, Debug, PartialEq)]
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

impl std::convert::From<String> for ItemType {
    fn from(itemtype_str: String) -> ItemType {
        match itemtype_str.as_str() {
            "S" => Self::Set,
            "P" => Self::Part,
            "M" => Self::Minifig,
            "B" => Self::Book,
            "G" => Self::Gear,
            "C" => Self::Catalog,
            "I" => Self::Instruction,
            "O" => Self::OriginalBox,
            "U" => Self::UnsortedLot,
            unsupported => panic!(format!("{} is not a supported ItemType!", unsupported)),
        }
    }
}

impl std::convert::From<ItemType> for String {
    fn from(item_type: ItemType) -> String {
        match item_type {
            ItemType::Set => "S".to_string(),
            ItemType::Part => "P".to_string(),
            ItemType::Minifig => "M".to_string(),
            ItemType::Book => "B".to_string(),
            ItemType::Gear => "G".to_string(),
            ItemType::Catalog => "C".to_string(),
            ItemType::Instruction => "I".to_string(),
            ItemType::OriginalBox => "O".to_string(),
            ItemType::UnsortedLot => "U".to_string(),
        }
    }
}

/// The canonical Lego catalog item number
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct MaxPrice(pub f32);

impl std::convert::From<String> for MaxPrice {
    fn from(input_string: String) -> MaxPrice {
        match input_string.parse::<f32>() {
            Ok(max_price) => return Self(max_price),
            Err(_e) => panic!("Could not parse MaxPrice {}", input_string),
        };
    }
}

impl std::convert::From<MaxPrice> for String {
    fn from(max_price: MaxPrice) -> String {
        format!("{:.2}", max_price.0).to_string()
    }
}

/// Minimum desired quantity
#[derive(Clone, Debug, PartialEq)]
pub struct MinQty(pub i32);

impl std::convert::From<i32> for MinQty {
    fn from(input_i32: i32) -> MinQty {
        Self(input_i32)
    }
}

impl std::convert::From<MinQty> for i32 {
    fn from(min_qty: MinQty) -> i32 {
        min_qty.0
    }
}

/// Quantity of the item you already have
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    New,
    Used,
    Complete,
    Incomplete,
    Sealed,
    NotProvided,
}

impl std::convert::From<String> for Condition {
    fn from(condition_str: String) -> Condition {
        match condition_str.as_str() {
            "N" => Self::New,
            "U" => Self::Used,
            "C" => Self::Complete,
            "I" => Self::Incomplete,
            "S" => Self::Sealed,
            "X" => Self::NotProvided,
            unsupported => panic!(format!("{} is not a supported Condition!", unsupported)),
        }
    }
}

impl std::convert::From<Condition> for String {
    fn from(condition: Condition) -> String {
        match condition {
            Condition::New => "N".to_string(),
            Condition::Used => "U".to_string(),
            Condition::Complete => "C".to_string(),
            Condition::Incomplete => "I".to_string(),
            Condition::Sealed => "S".to_string(),
            Condition::NotProvided => "X".to_string(),
        }
    }
}

/// Notes on the item
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub enum Notify {
    Y,
    N,
}

impl std::convert::From<String> for Notify {
    fn from(notify_str: String) -> Notify {
        match notify_str.as_str() {
            "Y" => Self::Y,
            "N" => Self::N,
            unsupported => panic!(format!("{} is not a supported Notify!", unsupported)),
        }
    }
}

impl std::convert::From<Notify> for String {
    fn from(notify: Notify) -> String {
        match notify {
            Notify::Y => "Y".to_string(),
            Notify::N => "N".to_string(),
        }
    }
}

/// Show in items for sale queries?
#[derive(Clone, Debug, PartialEq)]
pub enum WantedShow {
    Y,
    N,
}

impl std::convert::From<String> for WantedShow {
    fn from(wantedshow_str: String) -> WantedShow {
        match wantedshow_str.as_str() {
            "Y" => Self::Y,
            "N" => Self::N,
            unsupported => panic!(format!("{} is not a supported WantedShow!", unsupported)),
        }
    }
}

impl std::convert::From<WantedShow> for String {
    fn from(wantedshow: WantedShow) -> String {
        match wantedshow {
            WantedShow::Y => "Y".to_string(),
            WantedShow::N => "N".to_string(),
        }
    }
}

/// ID of wanted_list
#[derive(Clone, Debug, PartialEq)]
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
