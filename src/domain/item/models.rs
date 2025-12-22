use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    // pub icon_path: String, // DB doesn't have it in seed? Check seed.
    // Seed: (name, description, item_type, sub_type, rarity, level_requirement, stat_bonuses, buy_price, sell_price, max_stack, can_be_enhanced, max_enhancement)
    // No icon_path in seed!
    // But original code has icon_path.
    // I will add icon_path as Option or generated.
    #[cfg_attr(feature = "ssr", sqlx(default))]
    pub icon_path: Option<String>, 
    
    pub item_type: ItemType,
    // sub_type in DB, mapped to what? Original doesn't have sub_type field in struct, only ItemType enum.
    // Original ItemType has Weapon, Armor etc.
    // DB has item_type AND sub_type.
    // I should map DB item_type (string) to Enum.
    #[cfg_attr(feature = "ssr", sqlx(rename = "item_type"))]
    pub item_type_enum: ItemType, // This might fail if types don't match.
    // sqlx can map enum to postgres enum if defined.
    // seed has 'weapon', 'armor' strings.
    
    pub rarity: ItemRarity,
    pub level_requirement: i32,
    
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub stat_bonuses: HashMap<String, i32>,
    
    pub buy_price: i32,
    pub sell_price: i32,
    pub max_stack: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerItem {
    pub item: Item,
    pub quantity: i32,
    pub enhancement_level: i32,
    pub equipped: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "item_type", rename_all = "lowercase"))]
pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    Material,
    Quest,
}

impl ItemType {
    pub fn name(&self) -> &str {
        match self {
            ItemType::Weapon => "무기",
            ItemType::Armor => "방어구",
            ItemType::Accessory => "악세서리",
            ItemType::Consumable => "소비",
            ItemType::Material => "재료",
            ItemType::Quest => "퀘스트",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "item_rarity", rename_all = "lowercase"))]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Unique,
}

impl ItemRarity {
    pub fn color(&self) -> &str {
        match self {
            ItemRarity::Common => "#ffffff",
            ItemRarity::Uncommon => "#1eff00",
            ItemRarity::Rare => "#0070dd",
            ItemRarity::Epic => "#a335ee",
            ItemRarity::Legendary => "#ff8000",
            ItemRarity::Unique => "#e6cc80",
        }
    }
}
