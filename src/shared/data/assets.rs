//! Asset path constants
//!
//! Centralized asset path management for sprites, audio, and other resources.
//! All paths are relative to the public directory.

/// Base paths for different asset categories
pub mod paths {
    /// Base path for all assets
    pub const ASSETS_BASE: &str = "/assets";
    
    /// Map assets (village and dungeon tilesets)
    pub const MAPS_BASE: &str = "/assets/maps";
    
    /// Tile assets
    pub const TILES_BASE: &str = "/assets/tiles";
    pub const TILES_GROUND: &str = "/assets/tiles/ground";
    pub const TILES_BUILDINGS: &str = "/assets/tiles/buildings";
    pub const TILES_DECORATIONS: &str = "/assets/tiles/decorations";
    
    /// Character base sprites (Paper Doll body layer)
    pub const CHARACTERS_BASE: &str = "/assets/characters/base";
    
    /// Equipment layers (Paper Doll overlays)
    pub const EQUIPMENT_BASE: &str = "/assets/equipment";
    pub const EQUIPMENT_WEAPONS: &str = "/assets/equipment/weapons";
    pub const EQUIPMENT_ARMOR: &str = "/assets/equipment/armor";
    pub const EQUIPMENT_HELMETS: &str = "/assets/equipment/helmets";
    pub const EQUIPMENT_SHIELDS: &str = "/assets/equipment/shields";
    pub const EQUIPMENT_PANTS: &str = "/assets/equipment/pants";
    pub const EQUIPMENT_CAPES: &str = "/assets/equipment/capes";
    
    /// Monster assets
    pub const MONSTERS_BASE: &str = "/assets/monsters";
    
    /// UI assets
    pub const UI_BASE: &str = "/assets/ui";
    
    /// Audio assets
    pub const AUDIO_BASE: &str = "/assets/audio";
    
    /// Font assets
    pub const FONTS_BASE: &str = "/assets/fonts";
    
    /// Skill icon assets
    pub const SKILLS_BASE: &str = "/assets/skills";
    
    /// Item icon assets
    pub const ITEMS_BASE: &str = "/assets/items";
}

// ============================================================
// Paper Doll System - Base Character Sprites
// ============================================================

/// Base character body sprites (without any equipment)
pub mod base_characters {
    /// Male base body (simple shirt + shorts)
    pub const MALE_BODY: &str = "/assets/characters/base/male/body.png";
    
    /// Female base body (simple shirt + shorts)
    pub const FEMALE_BODY: &str = "/assets/characters/base/female/body.png";
    
    /// Get body sprite path by gender
    pub fn get_body_path(gender: &str) -> &'static str {
        match gender {
            "female" => FEMALE_BODY,
            _ => MALE_BODY,
        }
    }
}

// ============================================================
// Paper Doll System - Equipment Layer Sprites
// ============================================================

/// Equipment layer sprites (overlay on base character)
pub mod equipment_sprites {
    /// Weapon layer sprites (256x256, 4x4 grid matching character animation)
    pub mod weapons {
        pub const WOODEN_SWORD: &str = "/assets/equipment/weapons/wooden_sword.png";
        pub const IRON_SWORD: &str = "/assets/equipment/weapons/iron_sword.png";
        pub const STEEL_SWORD: &str = "/assets/equipment/weapons/steel_sword.png";
        pub const FIRE_SWORD: &str = "/assets/equipment/weapons/fire_sword.png";
        pub const RUSTY_DAGGER: &str = "/assets/equipment/weapons/rusty_dagger.png";
        pub const IRON_DAGGER: &str = "/assets/equipment/weapons/iron_dagger.png";
        pub const WOODEN_STAFF: &str = "/assets/equipment/weapons/wooden_staff.png";
        pub const MAGIC_STAFF: &str = "/assets/equipment/weapons/magic_staff.png";
    }
    
    /// Armor layer sprites (chest/torso overlay)
    pub mod armor {
        pub const LEATHER_ARMOR: &str = "/assets/equipment/armor/leather_armor.png";
        pub const CHAIN_MAIL: &str = "/assets/equipment/armor/chain_mail.png";
        pub const PLATE_ARMOR: &str = "/assets/equipment/armor/plate_armor.png";
        pub const MAGE_ROBE: &str = "/assets/equipment/armor/mage_robe.png";
        pub const CLERIC_VESTMENTS: &str = "/assets/equipment/armor/cleric_vestments.png";
    }
    
    /// Helmet layer sprites (head overlay)
    pub mod helmets {
        pub const LEATHER_CAP: &str = "/assets/equipment/helmets/leather_cap.png";
        pub const IRON_HELMET: &str = "/assets/equipment/helmets/iron_helmet.png";
        pub const STEEL_HELMET: &str = "/assets/equipment/helmets/steel_helmet.png";
        pub const WIZARD_HAT: &str = "/assets/equipment/helmets/wizard_hat.png";
    }
    
    /// Shield layer sprites (left hand overlay)
    pub mod shields {
        pub const WOODEN_SHIELD: &str = "/assets/equipment/shields/wooden_shield.png";
        pub const IRON_SHIELD: &str = "/assets/equipment/shields/iron_shield.png";
        pub const STEEL_SHIELD: &str = "/assets/equipment/shields/steel_shield.png";
    }
}

// ============================================================
// Tileset Configuration
// ============================================================

/// Ground tileset sprite sheet configuration
pub struct TilesetConfig {
    pub path: &'static str,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
}

/// Tile indices in the tileset spritesheet
pub mod tile_indices {
    // Row 0: Basic ground tiles
    pub const GRASS: u32 = 0;
    pub const GRASS_DARK: u32 = 1;
    pub const STONE: u32 = 2;
    pub const STONE_DARK: u32 = 3;
    pub const COBBLESTONE: u32 = 4;
    pub const DIRT: u32 = 5;
    
    // Row 1: Special tiles
    pub const WATER: u32 = 8;
    pub const WATER_EDGE: u32 = 9;
    pub const LAVA: u32 = 10;
    pub const ICE: u32 = 11;
    
    // Row 2: Wall tiles
    pub const WALL_STONE: u32 = 16;
    pub const WALL_BRICK: u32 = 17;
    pub const WALL_WOOD: u32 = 18;
    pub const DOOR_CLOSED: u32 = 19;
    pub const DOOR_OPEN: u32 = 20;
}

pub const GROUND_TILESET: TilesetConfig = TilesetConfig {
    path: "/assets/tiles/ground/tileset.png",
    tile_width: 64,
    tile_height: 32, // Isometric 2:1 ratio
    columns: 8,
    rows: 8,
};

// ============================================================
// Map Assets
// ============================================================

/// Village map assets (dark fantasy style)
pub mod village_map {
    pub const TILESET: &str = "/assets/maps/village/tileset.png";
    pub const BUILDINGS: &str = "/assets/maps/village/buildings.png";
    pub const MAP_DATA: &str = "/assets/maps/milles.json";
}

// ============================================================
// Building Configuration
// ============================================================

pub struct BuildingConfig {
    pub path: &'static str,
    pub width: u32,
    pub height: u32,
}

pub const BUILDINGS_SPRITESHEET: &str = "/assets/tiles/buildings/buildings.png";

pub mod building_indices {
    pub const HOUSE_SMALL: u32 = 0;
    pub const HOUSE_MEDIUM: u32 = 1;
    pub const BLACKSMITH: u32 = 2;
    pub const TAVERN: u32 = 3;
    pub const GUILD_HALL: u32 = 4;
    pub const SHOP: u32 = 5;
}

// ============================================================
// Decoration Sprites
// ============================================================

pub mod decorations {
    pub const TORCH: &str = "/assets/decorations/torch.png";
    pub const FOUNTAIN: &str = "/assets/decorations/fountain.png";
    pub const TREE: &str = "/assets/decorations/tree.png";
    pub const BUSH: &str = "/assets/decorations/bush.png";
    pub const ROCK: &str = "/assets/decorations/rock.png";
    pub const GRAVE: &str = "/assets/decorations/gravestone.png";
    pub const BARREL: &str = "/assets/decorations/barrel.png";
    pub const CRATE: &str = "/assets/decorations/crate.png";
    pub const CHEST: &str = "/assets/decorations/chest.png";
    pub const PORTAL: &str = "/assets/decorations/portal.png";
    pub const DOOR_WOODEN: &str = "/assets/decorations/door_wooden.png";
    pub const DOOR_DUNGEON: &str = "/assets/decorations/door_dungeon.png";
}

// ============================================================
// Font Configuration
// ============================================================

/// Font paths - Using Cinzel (medieval serif) for titles, NanumGothic for body text
pub mod fonts {
    /// Korean body text font
    pub const DEFAULT: &str = "/assets/fonts/NanumGothic.ttf";
    
    /// Medieval serif font for game title and headers (DND style)
    pub const TITLE: &str = "/assets/fonts/Cinzel-Bold.ttf";
    
    /// Medieval serif font for subtitles and important text
    pub const SUBTITLE: &str = "/assets/fonts/Cinzel-Regular.ttf";
    
    /// Get title font for medieval fantasy aesthetic
    pub const MEDIEVAL_TITLE: &str = "/assets/fonts/Cinzel-Bold.ttf";
}

// ============================================================
// Audio Configuration
// ============================================================

pub mod audio {
    pub const BGM_VILLAGE: &str = "/assets/audio/bgm/village.mp3";
    pub const BGM_DUNGEON: &str = "/assets/audio/bgm/dungeon.mp3";
    pub const BGM_BATTLE: &str = "/assets/audio/bgm/battle.mp3";
    pub const SFX_ATTACK: &str = "/assets/audio/sfx/attack.wav";
    pub const SFX_HIT: &str = "/assets/audio/sfx/hit.wav";
    pub const SFX_DEATH: &str = "/assets/audio/sfx/death.wav";
    pub const SFX_LEVEL_UP: &str = "/assets/audio/sfx/levelup.wav";
}

// ============================================================
// UI Assets
// ============================================================

pub mod ui {
    pub const BUTTON: &str = "/assets/ui/button.png";
    pub const PANEL: &str = "/assets/ui/panel.png";
    pub const HEALTH_BAR: &str = "/assets/ui/health_bar.png";
    pub const MANA_BAR: &str = "/assets/ui/mana_bar.png";
    pub const EXP_BAR: &str = "/assets/ui/exp_bar.png";
    pub const HP_ORB: &str = "/assets/ui/hp_orb.png";
    pub const MP_ORB: &str = "/assets/ui/mp_orb.png";
    pub const INVENTORY_SLOT: &str = "/assets/ui/inventory_slot.png";
    pub const SKILL_SLOT: &str = "/assets/ui/skill_slot.png";
    pub const EQUIPMENT_SLOT: &str = "/assets/ui/equipment_slot.png";
    pub const SLOT: &str = "/assets/ui/slot.png";
    pub const MINIMAP_FRAME: &str = "/assets/ui/minimap_frame.png";
    pub const QUICKBAR_BG: &str = "/assets/ui/quickbar_bg.png";
    pub const LOGO: &str = "/assets/ui/logo.png";
}

// ============================================================
// Combat Effect Assets
// ============================================================

pub mod effects {
    pub const SLASH: &str = "/assets/effects/slash.png";
    pub const FIRE: &str = "/assets/effects/fire.png";
    pub const ICE: &str = "/assets/effects/ice.png";
    pub const LIGHTNING: &str = "/assets/effects/lightning.png";
    pub const HEAL: &str = "/assets/effects/heal.png";
    pub const POISON: &str = "/assets/effects/poison.png";
    pub const DARK: &str = "/assets/effects/dark.png";
    pub const HOLY: &str = "/assets/effects/holy.png";
    pub const STUN: &str = "/assets/effects/stun.png";
    pub const BLEED: &str = "/assets/effects/bleed.png";
    pub const SHIELD: &str = "/assets/effects/shield.png";
}

// ============================================================
// Terrain Tile Assets
// ============================================================

pub mod tiles {
    pub const GRASS: &str = "/assets/tiles/grass.png";
    pub const STONE: &str = "/assets/tiles/stone.png";
    pub const DIRT: &str = "/assets/tiles/dirt.png";
    pub const WATER: &str = "/assets/tiles/water.png";
    pub const WALL: &str = "/assets/tiles/wall.png";
    pub const LAVA: &str = "/assets/tiles/lava.png";
    pub const SAND: &str = "/assets/tiles/sand.png";
    pub const ICE: &str = "/assets/tiles/ice.png";
    pub const LAVA_ROCK: &str = "/assets/tiles/lava_rock.png";
    pub const DUNGEON: &str = "/assets/tiles/dungeon.png";
    pub const CARPET: &str = "/assets/tiles/carpet.png";
    pub const BRIDGE: &str = "/assets/tiles/bridge.png";
}

// ============================================================
// Background Assets
// ============================================================

pub mod backgrounds {
    pub const LOGIN: &str = "/assets/backgrounds/login.png";
    pub const CHARACTER_SELECT: &str = "/assets/backgrounds/character_select.png";
}

// ============================================================
// Cursor Assets
// ============================================================

pub mod cursors {
    pub const NORMAL: &str = "/assets/cursors/normal.png";
    pub const ATTACK: &str = "/assets/cursors/attack.png";
    pub const INTERACT: &str = "/assets/cursors/interact.png";
}

// ============================================================
// Icon Assets
// ============================================================

pub mod icons {
    pub const QUEST: &str = "/assets/icons/quest.png";
    pub const QUEST_COMPLETE: &str = "/assets/icons/quest_complete.png";
    pub const LEVEL_UP: &str = "/assets/icons/level_up.png";
    pub const BUFF: &str = "/assets/icons/buff.png";
    pub const DEBUFF: &str = "/assets/icons/debuff.png";
    pub const DEATH: &str = "/assets/icons/death.png";
}

