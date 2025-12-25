//! Map data constants - 5 Circle System
//!
//! All map definitions organized by circle regions.

/// Map definition
#[derive(Debug, Clone)]
pub struct MapDef {
    pub id: &'static str,
    pub name: &'static str,
    pub name_key: &'static str,
    pub circle: i32,
    pub width: usize,
    pub height: usize,
    pub min_level: i32,
    pub max_level: i32,
    pub is_town: bool,
    pub is_dungeon: bool,
    pub has_boss: bool,
    pub bgm_path: &'static str,
}

/// Tile layout for maps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTile {
    Grass,      // 'G' - Walkable
    Stone,      // 'S' - Walkable
    Water,      // 'W' - Non-walkable
    Wall,       // 'X' - Non-walkable
    Sand,       // 'A' - Walkable (desert)
    Ice,        // 'I' - Walkable (slippery)
    Lava,       // 'L' - Damage tile
    Tree,       // 'T' - Non-walkable
    Building,   // 'B' - Non-walkable
    Door,       // 'D' - Portal
}

impl MapTile {
    pub fn from_char(c: char) -> Self {
        match c {
            'G' => MapTile::Grass,
            'S' => MapTile::Stone,
            'W' => MapTile::Water,
            'X' => MapTile::Wall,
            'A' => MapTile::Sand,
            'I' => MapTile::Ice,
            'L' => MapTile::Lava,
            'T' => MapTile::Tree,
            'B' => MapTile::Building,
            'D' => MapTile::Door,
            _ => MapTile::Grass,
        }
    }
    
    pub fn is_walkable(&self) -> bool {
        matches!(self, MapTile::Grass | MapTile::Stone | MapTile::Sand | MapTile::Ice | MapTile::Door)
    }
}

/// Monster spawn point
#[derive(Debug, Clone)]
pub struct SpawnPoint {
    pub x: i32,
    pub y: i32,
    pub monster_id: i32,
    pub respawn_time_ms: u64,
}

/// NPC definition
#[derive(Debug, Clone)]
pub struct NpcDef {
    pub id: &'static str,
    pub name_key: &'static str,
    pub x: i32,
    pub y: i32,
    pub npc_type: NpcType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    Shop,
    Inn,
    Blacksmith,
    QuestGiver,
    ClassTrainer,
    Portal,
}

/// Portal definition
#[derive(Debug, Clone)]
pub struct PortalDef {
    pub x: i32,
    pub y: i32,
    pub target_map: &'static str,
    pub target_x: i32,
    pub target_y: i32,
}

// ============================================================
// CIRCLE 1: MILLES REGION (Lv 1-20)
// ============================================================

pub const MILLES_VILLAGE: MapDef = MapDef {
    id: "milles_village", name: "Milles Village", name_key: "map.milles_village",
    circle: 1, width: 32, height: 32, min_level: 1, max_level: 99,
    is_town: true, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/village.mp3",
};

pub const MILLES_PLAINS: MapDef = MapDef {
    id: "milles_plains", name: "Milles Plains", name_key: "map.milles_plains",
    circle: 1, width: 64, height: 64, min_level: 1, max_level: 10,
    is_town: false, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/field.mp3",
};

pub const WOLF_FOREST: MapDef = MapDef {
    id: "wolf_forest", name: "Wolf Forest", name_key: "map.wolf_forest",
    circle: 1, width: 48, height: 48, min_level: 10, max_level: 20,
    is_town: false, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/forest.mp3",
};

pub const WOLF_DEN: MapDef = MapDef {
    id: "wolf_den", name: "Wolf Den", name_key: "map.wolf_den",
    circle: 1, width: 24, height: 24, min_level: 15, max_level: 20,
    is_town: false, is_dungeon: true, has_boss: true,
    bgm_path: "/assets/audio/bgm/dungeon.mp3",
};

// ============================================================
// CIRCLE 2: SARAKH REGION (Lv 21-40)
// ============================================================

pub const SARAKH_OASIS: MapDef = MapDef {
    id: "sarakh_oasis", name: "Sarakh Oasis", name_key: "map.sarakh_oasis",
    circle: 2, width: 32, height: 32, min_level: 21, max_level: 99,
    is_town: true, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/oasis.mp3",
};

pub const SARAKH_DESERT: MapDef = MapDef {
    id: "sarakh_desert", name: "Sarakh Desert", name_key: "map.sarakh_desert",
    circle: 2, width: 80, height: 80, min_level: 21, max_level: 35,
    is_town: false, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/desert.mp3",
};

pub const ANCIENT_PYRAMID: MapDef = MapDef {
    id: "ancient_pyramid", name: "Ancient Pyramid", name_key: "map.ancient_pyramid",
    circle: 2, width: 32, height: 32, min_level: 30, max_level: 40,
    is_town: false, is_dungeon: true, has_boss: true,
    bgm_path: "/assets/audio/bgm/pyramid.mp3",
};

// ============================================================
// CIRCLE 3: FROST REGION (Lv 41-60)
// ============================================================

pub const FROST_HAVEN: MapDef = MapDef {
    id: "frost_haven", name: "Frost Haven", name_key: "map.frost_haven",
    circle: 3, width: 32, height: 32, min_level: 41, max_level: 99,
    is_town: true, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/frost_town.mp3",
};

pub const FROST_MOUNTAIN: MapDef = MapDef {
    id: "frost_mountain", name: "Frost Mountain", name_key: "map.frost_mountain",
    circle: 3, width: 64, height: 64, min_level: 41, max_level: 55,
    is_town: false, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/blizzard.mp3",
};

pub const ICE_CAVERN: MapDef = MapDef {
    id: "ice_cavern", name: "Ice Cavern", name_key: "map.ice_cavern",
    circle: 3, width: 40, height: 40, min_level: 50, max_level: 60,
    is_town: false, is_dungeon: true, has_boss: true,
    bgm_path: "/assets/audio/bgm/ice_cave.mp3",
};

// ============================================================
// CIRCLE 4: INFERNO REGION (Lv 61-80)
// ============================================================

pub const EMBER_OUTPOST: MapDef = MapDef {
    id: "ember_outpost", name: "Ember Outpost", name_key: "map.ember_outpost",
    circle: 4, width: 24, height: 24, min_level: 61, max_level: 99,
    is_town: true, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/outpost.mp3",
};

pub const INFERNO_VOLCANO: MapDef = MapDef {
    id: "inferno_volcano", name: "Inferno Volcano", name_key: "map.inferno_volcano",
    circle: 4, width: 64, height: 64, min_level: 61, max_level: 75,
    is_town: false, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/volcano.mp3",
};

pub const DEMON_LAIR: MapDef = MapDef {
    id: "demon_lair", name: "Demon Lair", name_key: "map.demon_lair",
    circle: 4, width: 48, height: 48, min_level: 70, max_level: 80,
    is_town: false, is_dungeon: true, has_boss: true,
    bgm_path: "/assets/audio/bgm/demon.mp3",
};

// ============================================================
// CIRCLE 5: DARK REGION (Lv 81-99)
// ============================================================

pub const SHADOW_SANCTUARY: MapDef = MapDef {
    id: "shadow_sanctuary", name: "Shadow Sanctuary", name_key: "map.shadow_sanctuary",
    circle: 5, width: 24, height: 24, min_level: 81, max_level: 99,
    is_town: true, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/sanctuary.mp3",
};

pub const DARK_CASTLE: MapDef = MapDef {
    id: "dark_castle", name: "Dark Castle", name_key: "map.dark_castle",
    circle: 5, width: 80, height: 80, min_level: 81, max_level: 95,
    is_town: false, is_dungeon: false, has_boss: false,
    bgm_path: "/assets/audio/bgm/dark_castle.mp3",
};

pub const THRONE_OF_DARKNESS: MapDef = MapDef {
    id: "throne_of_darkness", name: "Throne of Darkness", name_key: "map.throne_of_darkness",
    circle: 5, width: 32, height: 32, min_level: 90, max_level: 99,
    is_town: false, is_dungeon: true, has_boss: true,
    bgm_path: "/assets/audio/bgm/final_boss.mp3",
};

// ============================================================
// MAP REGISTRY
// ============================================================

pub const ALL_MAPS: &[&MapDef] = &[
    // Circle 1
    &MILLES_VILLAGE, &MILLES_PLAINS, &WOLF_FOREST, &WOLF_DEN,
    // Circle 2
    &SARAKH_OASIS, &SARAKH_DESERT, &ANCIENT_PYRAMID,
    // Circle 3
    &FROST_HAVEN, &FROST_MOUNTAIN, &ICE_CAVERN,
    // Circle 4
    &EMBER_OUTPOST, &INFERNO_VOLCANO, &DEMON_LAIR,
    // Circle 5
    &SHADOW_SANCTUARY, &DARK_CASTLE, &THRONE_OF_DARKNESS,
];

pub fn get_map_by_id(id: &str) -> Option<&'static MapDef> {
    ALL_MAPS.iter().find(|m| m.id == id).copied()
}

pub fn get_maps_by_circle(circle: i32) -> Vec<&'static MapDef> {
    ALL_MAPS.iter().filter(|m| m.circle == circle).copied().collect()
}

pub fn get_towns() -> Vec<&'static MapDef> {
    ALL_MAPS.iter().filter(|m| m.is_town).copied().collect()
}

pub fn get_dungeons() -> Vec<&'static MapDef> {
    ALL_MAPS.iter().filter(|m| m.is_dungeon).copied().collect()
}

// ============================================================
// SPAWN CONFIGURATIONS
// ============================================================

pub const MILLES_PLAINS_SPAWNS: &[SpawnPoint] = &[
    SpawnPoint { x: 10, y: 10, monster_id: 101, respawn_time_ms: 30000 },  // Giant Rat
    SpawnPoint { x: 20, y: 15, monster_id: 101, respawn_time_ms: 30000 },
    SpawnPoint { x: 30, y: 20, monster_id: 102, respawn_time_ms: 30000 },  // Vampire Bat
    SpawnPoint { x: 15, y: 30, monster_id: 103, respawn_time_ms: 45000 },  // Slime
    SpawnPoint { x: 40, y: 25, monster_id: 104, respawn_time_ms: 60000 },  // Corrupted Fox
];

pub const WOLF_FOREST_SPAWNS: &[SpawnPoint] = &[
    SpawnPoint { x: 10, y: 10, monster_id: 104, respawn_time_ms: 45000 },  // Corrupted Fox
    SpawnPoint { x: 25, y: 20, monster_id: 105, respawn_time_ms: 60000 },  // Wolf
    SpawnPoint { x: 35, y: 15, monster_id: 105, respawn_time_ms: 60000 },
    SpawnPoint { x: 20, y: 35, monster_id: 105, respawn_time_ms: 60000 },
];

pub const WOLF_DEN_SPAWNS: &[SpawnPoint] = &[
    SpawnPoint { x: 5, y: 5, monster_id: 105, respawn_time_ms: 45000 },    // Wolf
    SpawnPoint { x: 15, y: 10, monster_id: 105, respawn_time_ms: 45000 },
    SpawnPoint { x: 12, y: 12, monster_id: 199, respawn_time_ms: 300000 }, // Wolf Alpha (Boss)
];

// ============================================================
// NPC CONFIGURATIONS
// ============================================================

pub const MILLES_NPCS: &[NpcDef] = &[
    NpcDef { id: "innkeeper", name_key: "npc.innkeeper", x: 8, y: 8, npc_type: NpcType::Inn },
    NpcDef { id: "shopkeeper", name_key: "npc.shopkeeper", x: 12, y: 8, npc_type: NpcType::Shop },
    NpcDef { id: "blacksmith", name_key: "npc.blacksmith", x: 16, y: 8, npc_type: NpcType::Blacksmith },
    NpcDef { id: "warrior_trainer", name_key: "npc.warrior_trainer", x: 10, y: 16, npc_type: NpcType::ClassTrainer },
    NpcDef { id: "mage_trainer", name_key: "npc.mage_trainer", x: 14, y: 16, npc_type: NpcType::ClassTrainer },
    NpcDef { id: "quest_elder", name_key: "npc.quest_elder", x: 16, y: 16, npc_type: NpcType::QuestGiver },
];

// ============================================================
// PORTAL CONFIGURATIONS
// ============================================================

pub const MILLES_VILLAGE_PORTALS: &[PortalDef] = &[
    PortalDef { x: 16, y: 30, target_map: "milles_plains", target_x: 5, target_y: 5 },
];

pub const MILLES_PLAINS_PORTALS: &[PortalDef] = &[
    PortalDef { x: 5, y: 5, target_map: "milles_village", target_x: 16, target_y: 28 },
    PortalDef { x: 60, y: 60, target_map: "wolf_forest", target_x: 5, target_y: 5 },
];
