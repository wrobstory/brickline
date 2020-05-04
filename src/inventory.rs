//! A Bricklink Inventory
//!
//! These types are all based on the Bricklink
//! XML schema as described here: https://www.bricklink.com/help.asp?helpID=207

/// The top level inventory that will hold a vector of Items
struct Inventory {
    items: Vec<Item>
}

/// A single Lego Item
struct Item;

/// The type of the Lego Item
enum ItemType {
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

/// The canonical Lego catalog item number
struct ItemNumber(String);

/// Color ID according to the Bricklink color catalog 
/// https://www.bricklink.com/catalogColors.asp
struct Color(i8);

/// Maximum Desired Price
struct MaxPrice(i32);

/// Minimum desired quantity
struct MinQty(i32);

/// Quantity of the item you already have
struct QtyFilled(i32);

/// Item condition
enum Condition {
    New, 
    Used, 
    Complete, 
    Incomplete, 
    Sealed
}

/// Notes on the item
struct Remarks(String);

/// Be notified when these items are listed for sale
enum Notify {
	Y,
	N
}

/// Show in items for sale queries? 
enum WantedShow {
	Y, 
	N
}


/// ID of wanted list
struct WantedListID(String);




