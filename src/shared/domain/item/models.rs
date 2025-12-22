use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Armor,
    Shoes,
    Gloves,
    Ring1,
    Ring2,
    Necklace,
    Weapon,
    Shield,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: i32, // DB definition ID
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub sub_type: String,
    pub grade: i32, // 1-12
    pub req_level: i32,
    pub stats: serde_json::Value, // Bonus stats
    pub icon_path: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    Etc,
}
