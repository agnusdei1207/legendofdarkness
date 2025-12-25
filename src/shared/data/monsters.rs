//! Monster data constants - 5 Circle System
//!
//! All monster definitions organized by circle and region.
//! Circle 1: Lv 1-20, Circle 2: Lv 21-40, Circle 3: Lv 41-60, Circle 4: Lv 61-80, Circle 5: Lv 81-99

use crate::shared::domain::monster::{MonsterAIType, SpriteSize};

/// Monster definition constant data
#[derive(Debug, Clone)]
pub struct MonsterDef {
    pub id: i32,
    pub name: &'static str,
    pub name_key: &'static str,
    pub circle: i32,            // 1-5
    pub level: i32,
    pub hp_max: i32,
    pub mp_max: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
    pub ai_type: MonsterAIType,
    pub detection_range: f64,
    pub attack_range: f64,
    pub move_speed: f64,
    pub sprite_type: &'static str,
    pub sprite_size: SpriteSize,
    pub is_boss: bool,
    pub region: &'static str,
}

impl MonsterDef {
    pub fn sprite_path(&self) -> String {
        format!("/assets/monsters/{}/spritesheet.png", self.sprite_type)
    }
}

// ============================================================
// CIRCLE 1: MILLES PLAINS (Lv 1-20)
// ============================================================

pub const GIANT_RAT: MonsterDef = MonsterDef {
    id: 101, name: "Giant Rat", name_key: "monster.giant_rat",
    circle: 1, level: 1, hp_max: 20, mp_max: 0,
    attack_min: 2, attack_max: 4, defense: 0,
    exp_reward: 5, gold_min: 1, gold_max: 3,
    ai_type: MonsterAIType::Aggressive, detection_range: 250.0,
    attack_range: 55.0, move_speed: 110.0,
    sprite_type: "rat", sprite_size: SpriteSize::Small,
    is_boss: false, region: "milles_plains",
};

pub const VAMPIRE_BAT: MonsterDef = MonsterDef {
    id: 102, name: "Vampire Bat", name_key: "monster.vampire_bat",
    circle: 1, level: 3, hp_max: 35, mp_max: 0,
    attack_min: 4, attack_max: 6, defense: 1,
    exp_reward: 10, gold_min: 2, gold_max: 5,
    ai_type: MonsterAIType::Aggressive, detection_range: 250.0,
    attack_range: 55.0, move_speed: 120.0,
    sprite_type: "bat", sprite_size: SpriteSize::Small,
    is_boss: false, region: "milles_plains",
};

pub const SLIME: MonsterDef = MonsterDef {
    id: 103, name: "Slime", name_key: "monster.slime",
    circle: 1, level: 5, hp_max: 50, mp_max: 0,
    attack_min: 6, attack_max: 9, defense: 2,
    exp_reward: 18, gold_min: 5, gold_max: 10,
    ai_type: MonsterAIType::Aggressive, detection_range: 150.0,
    attack_range: 40.0, move_speed: 60.0,
    sprite_type: "slime", sprite_size: SpriteSize::Small,
    is_boss: false, region: "milles_plains",
};

pub const CORRUPTED_FOX: MonsterDef = MonsterDef {
    id: 104, name: "Corrupted Fox", name_key: "monster.corrupted_fox",
    circle: 1, level: 10, hp_max: 120, mp_max: 20,
    attack_min: 15, attack_max: 20, defense: 5,
    exp_reward: 45, gold_min: 15, gold_max: 30,
    ai_type: MonsterAIType::Aggressive, detection_range: 200.0,
    attack_range: 50.0, move_speed: 100.0,
    sprite_type: "fox", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "milles_plains",
};

pub const WOLF: MonsterDef = MonsterDef {
    id: 105, name: "Wolf", name_key: "monster.wolf",
    circle: 1, level: 15, hp_max: 200, mp_max: 30,
    attack_min: 25, attack_max: 35, defense: 10,
    exp_reward: 80, gold_min: 25, gold_max: 50,
    ai_type: MonsterAIType::Aggressive, detection_range: 280.0,
    attack_range: 60.0, move_speed: 130.0,
    sprite_type: "wolf", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "milles_plains",
};

// Circle 1 Boss
pub const WOLF_ALPHA: MonsterDef = MonsterDef {
    id: 199, name: "Wolf Alpha", name_key: "monster.wolf_alpha",
    circle: 1, level: 20, hp_max: 1000, mp_max: 100,
    attack_min: 50, attack_max: 80, defense: 25,
    exp_reward: 500, gold_min: 200, gold_max: 500,
    ai_type: MonsterAIType::Defensive, detection_range: 350.0,
    attack_range: 70.0, move_speed: 120.0,
    sprite_type: "wolf_alpha", sprite_size: SpriteSize::Large,
    is_boss: true, region: "milles_plains",
};

// ============================================================
// CIRCLE 2: SARAKH DESERT (Lv 21-40)
// ============================================================

pub const SKELETON: MonsterDef = MonsterDef {
    id: 201, name: "Skeleton", name_key: "monster.skeleton",
    circle: 2, level: 22, hp_max: 350, mp_max: 0,
    attack_min: 40, attack_max: 55, defense: 15,
    exp_reward: 150, gold_min: 40, gold_max: 80,
    ai_type: MonsterAIType::Aggressive, detection_range: 220.0,
    attack_range: 55.0, move_speed: 90.0,
    sprite_type: "skeleton", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "sarakh_desert",
};

pub const DESERT_SCORPION: MonsterDef = MonsterDef {
    id: 202, name: "Desert Scorpion", name_key: "monster.desert_scorpion",
    circle: 2, level: 26, hp_max: 450, mp_max: 0,
    attack_min: 50, attack_max: 70, defense: 20,
    exp_reward: 200, gold_min: 50, gold_max: 100,
    ai_type: MonsterAIType::Aggressive, detection_range: 180.0,
    attack_range: 45.0, move_speed: 85.0,
    sprite_type: "scorpion", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "sarakh_desert",
};

pub const MUMMY: MonsterDef = MonsterDef {
    id: 203, name: "Mummy", name_key: "monster.mummy",
    circle: 2, level: 30, hp_max: 600, mp_max: 100,
    attack_min: 60, attack_max: 85, defense: 30,
    exp_reward: 280, gold_min: 70, gold_max: 140,
    ai_type: MonsterAIType::Aggressive, detection_range: 200.0,
    attack_range: 50.0, move_speed: 70.0,
    sprite_type: "mummy", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "sarakh_desert",
};

pub const SAND_GOLEM: MonsterDef = MonsterDef {
    id: 204, name: "Sand Golem", name_key: "monster.sand_golem",
    circle: 2, level: 35, hp_max: 900, mp_max: 0,
    attack_min: 80, attack_max: 110, defense: 50,
    exp_reward: 400, gold_min: 100, gold_max: 200,
    ai_type: MonsterAIType::Defensive, detection_range: 150.0,
    attack_range: 60.0, move_speed: 50.0,
    sprite_type: "golem", sprite_size: SpriteSize::Large,
    is_boss: false, region: "sarakh_desert",
};

// Circle 2 Boss
pub const SCORPION_KING: MonsterDef = MonsterDef {
    id: 299, name: "Scorpion King", name_key: "monster.scorpion_king",
    circle: 2, level: 40, hp_max: 5000, mp_max: 500,
    attack_min: 150, attack_max: 220, defense: 80,
    exp_reward: 3000, gold_min: 1000, gold_max: 2500,
    ai_type: MonsterAIType::Defensive, detection_range: 400.0,
    attack_range: 80.0, move_speed: 100.0,
    sprite_type: "scorpion_king", sprite_size: SpriteSize::Boss,
    is_boss: true, region: "sarakh_desert",
};

// ============================================================
// CIRCLE 3: FROST MOUNTAIN (Lv 41-60)
// ============================================================

pub const GOBLIN: MonsterDef = MonsterDef {
    id: 301, name: "Goblin", name_key: "monster.goblin",
    circle: 3, level: 42, hp_max: 800, mp_max: 50,
    attack_min: 90, attack_max: 120, defense: 40,
    exp_reward: 500, gold_min: 100, gold_max: 200,
    ai_type: MonsterAIType::Aggressive, detection_range: 250.0,
    attack_range: 50.0, move_speed: 100.0,
    sprite_type: "goblin", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "frost_mountain",
};

pub const FROST_WOLF: MonsterDef = MonsterDef {
    id: 302, name: "Frost Wolf", name_key: "monster.frost_wolf",
    circle: 3, level: 46, hp_max: 1000, mp_max: 100,
    attack_min: 110, attack_max: 150, defense: 50,
    exp_reward: 650, gold_min: 130, gold_max: 260,
    ai_type: MonsterAIType::Aggressive, detection_range: 300.0,
    attack_range: 60.0, move_speed: 140.0,
    sprite_type: "frost_wolf", sprite_size: SpriteSize::Medium,
    is_boss: false, region: "frost_mountain",
};

pub const GHOST: MonsterDef = MonsterDef {
    id: 303, name: "Ghost", name_key: "monster.ghost",
    circle: 3, level: 50, hp_max: 1200, mp_max: 200,
    attack_min: 130, attack_max: 170, defense: 30,
    exp_reward: 800, gold_min: 160, gold_max: 320,
    ai_type: MonsterAIType::Aggressive, detection_range: 300.0,
    attack_range: 70.0, move_speed: 80.0,
    sprite_type: "ghost", sprite_size: SpriteSize::Large,
    is_boss: false, region: "frost_mountain",
};

pub const YETI: MonsterDef = MonsterDef {
    id: 304, name: "Yeti", name_key: "monster.yeti",
    circle: 3, level: 55, hp_max: 2000, mp_max: 100,
    attack_min: 180, attack_max: 250, defense: 80,
    exp_reward: 1200, gold_min: 250, gold_max: 500,
    ai_type: MonsterAIType::Aggressive, detection_range: 280.0,
    attack_range: 70.0, move_speed: 90.0,
    sprite_type: "yeti", sprite_size: SpriteSize::Large,
    is_boss: false, region: "frost_mountain",
};

// Circle 3 Boss
pub const ICE_GOLEM: MonsterDef = MonsterDef {
    id: 399, name: "Ice Golem", name_key: "monster.ice_golem",
    circle: 3, level: 60, hp_max: 15000, mp_max: 1000,
    attack_min: 300, attack_max: 450, defense: 150,
    exp_reward: 8000, gold_min: 3000, gold_max: 6000,
    ai_type: MonsterAIType::Defensive, detection_range: 400.0,
    attack_range: 90.0, move_speed: 60.0,
    sprite_type: "ice_golem", sprite_size: SpriteSize::Boss,
    is_boss: true, region: "frost_mountain",
};

// ============================================================
// CIRCLE 4: INFERNO VOLCANO (Lv 61-80)
// ============================================================

pub const DARK_KNIGHT: MonsterDef = MonsterDef {
    id: 401, name: "Dark Knight", name_key: "monster.dark_knight",
    circle: 4, level: 62, hp_max: 3000, mp_max: 500,
    attack_min: 250, attack_max: 350, defense: 120,
    exp_reward: 2000, gold_min: 400, gold_max: 800,
    ai_type: MonsterAIType::Aggressive, detection_range: 350.0,
    attack_range: 80.0, move_speed: 90.0,
    sprite_type: "dark_knight", sprite_size: SpriteSize::Large,
    is_boss: false, region: "inferno_volcano",
};

pub const FIRE_ELEMENTAL: MonsterDef = MonsterDef {
    id: 402, name: "Fire Elemental", name_key: "monster.fire_elemental",
    circle: 4, level: 66, hp_max: 3500, mp_max: 800,
    attack_min: 280, attack_max: 400, defense: 100,
    exp_reward: 2500, gold_min: 500, gold_max: 1000,
    ai_type: MonsterAIType::Aggressive, detection_range: 300.0,
    attack_range: 90.0, move_speed: 100.0,
    sprite_type: "fire_elemental", sprite_size: SpriteSize::Large,
    is_boss: false, region: "inferno_volcano",
};

pub const HELL_HOUND: MonsterDef = MonsterDef {
    id: 403, name: "Hell Hound", name_key: "monster.hell_hound",
    circle: 4, level: 70, hp_max: 4000, mp_max: 300,
    attack_min: 320, attack_max: 450, defense: 130,
    exp_reward: 3000, gold_min: 600, gold_max: 1200,
    ai_type: MonsterAIType::Aggressive, detection_range: 400.0,
    attack_range: 70.0, move_speed: 150.0,
    sprite_type: "hell_hound", sprite_size: SpriteSize::Large,
    is_boss: false, region: "inferno_volcano",
};

pub const LAVA_GOLEM: MonsterDef = MonsterDef {
    id: 404, name: "Lava Golem", name_key: "monster.lava_golem",
    circle: 4, level: 75, hp_max: 6000, mp_max: 200,
    attack_min: 400, attack_max: 550, defense: 200,
    exp_reward: 5000, gold_min: 1000, gold_max: 2000,
    ai_type: MonsterAIType::Defensive, detection_range: 250.0,
    attack_range: 80.0, move_speed: 50.0,
    sprite_type: "lava_golem", sprite_size: SpriteSize::Large,
    is_boss: false, region: "inferno_volcano",
};

// Circle 4 Boss
pub const INFERNO_DEMON: MonsterDef = MonsterDef {
    id: 499, name: "Inferno Demon", name_key: "monster.inferno_demon",
    circle: 4, level: 80, hp_max: 30000, mp_max: 3000,
    attack_min: 600, attack_max: 900, defense: 300,
    exp_reward: 25000, gold_min: 8000, gold_max: 15000,
    ai_type: MonsterAIType::Defensive, detection_range: 500.0,
    attack_range: 120.0, move_speed: 80.0,
    sprite_type: "inferno_demon", sprite_size: SpriteSize::Boss,
    is_boss: true, region: "inferno_volcano",
};

// ============================================================
// CIRCLE 5: DARK CASTLE (Lv 81-99)
// ============================================================

pub const LICH: MonsterDef = MonsterDef {
    id: 501, name: "Lich", name_key: "monster.lich",
    circle: 5, level: 82, hp_max: 8000, mp_max: 5000,
    attack_min: 500, attack_max: 800, defense: 200,
    exp_reward: 10000, gold_min: 2000, gold_max: 5000,
    ai_type: MonsterAIType::Defensive, detection_range: 400.0,
    attack_range: 100.0, move_speed: 70.0,
    sprite_type: "lich", sprite_size: SpriteSize::Large,
    is_boss: false, region: "dark_castle",
};

pub const DEATH_KNIGHT: MonsterDef = MonsterDef {
    id: 502, name: "Death Knight", name_key: "monster.death_knight",
    circle: 5, level: 86, hp_max: 12000, mp_max: 2000,
    attack_min: 700, attack_max: 1000, defense: 350,
    exp_reward: 15000, gold_min: 3000, gold_max: 7000,
    ai_type: MonsterAIType::Aggressive, detection_range: 400.0,
    attack_range: 90.0, move_speed: 100.0,
    sprite_type: "death_knight", sprite_size: SpriteSize::Large,
    is_boss: false, region: "dark_castle",
};

pub const SHADOW_DRAGON: MonsterDef = MonsterDef {
    id: 503, name: "Shadow Dragon", name_key: "monster.shadow_dragon",
    circle: 5, level: 90, hp_max: 20000, mp_max: 3000,
    attack_min: 900, attack_max: 1300, defense: 400,
    exp_reward: 25000, gold_min: 5000, gold_max: 10000,
    ai_type: MonsterAIType::Aggressive, detection_range: 450.0,
    attack_range: 120.0, move_speed: 110.0,
    sprite_type: "shadow_dragon", sprite_size: SpriteSize::Boss,
    is_boss: false, region: "dark_castle",
};

pub const ARCH_LICH: MonsterDef = MonsterDef {
    id: 504, name: "Arch Lich", name_key: "monster.arch_lich",
    circle: 5, level: 95, hp_max: 30000, mp_max: 10000,
    attack_min: 1000, attack_max: 1500, defense: 300,
    exp_reward: 40000, gold_min: 8000, gold_max: 15000,
    ai_type: MonsterAIType::Defensive, detection_range: 500.0,
    attack_range: 150.0, move_speed: 60.0,
    sprite_type: "arch_lich", sprite_size: SpriteSize::Boss,
    is_boss: false, region: "dark_castle",
};

// Circle 5 Boss (Final Boss)
pub const DARK_LORD: MonsterDef = MonsterDef {
    id: 599, name: "Dark Lord", name_key: "monster.dark_lord",
    circle: 5, level: 99, hp_max: 100000, mp_max: 20000,
    attack_min: 2000, attack_max: 3500, defense: 600,
    exp_reward: 200000, gold_min: 50000, gold_max: 100000,
    ai_type: MonsterAIType::Defensive, detection_range: 600.0,
    attack_range: 180.0, move_speed: 100.0,
    sprite_type: "dark_lord", sprite_size: SpriteSize::Boss,
    is_boss: true, region: "dark_castle",
};

// ============================================================
// MONSTER REGISTRY
// ============================================================

pub const ALL_MONSTERS: &[&MonsterDef] = &[
    // Circle 1
    &GIANT_RAT, &VAMPIRE_BAT, &SLIME, &CORRUPTED_FOX, &WOLF, &WOLF_ALPHA,
    // Circle 2
    &SKELETON, &DESERT_SCORPION, &MUMMY, &SAND_GOLEM, &SCORPION_KING,
    // Circle 3
    &GOBLIN, &FROST_WOLF, &GHOST, &YETI, &ICE_GOLEM,
    // Circle 4
    &DARK_KNIGHT, &FIRE_ELEMENTAL, &HELL_HOUND, &LAVA_GOLEM, &INFERNO_DEMON,
    // Circle 5
    &LICH, &DEATH_KNIGHT, &SHADOW_DRAGON, &ARCH_LICH, &DARK_LORD,
];

pub fn get_monster_by_name(name: &str) -> Option<&'static MonsterDef> {
    ALL_MONSTERS.iter().find(|m| m.name == name).copied()
}

pub fn get_monster_by_id(id: i32) -> Option<&'static MonsterDef> {
    ALL_MONSTERS.iter().find(|m| m.id == id).copied()
}

pub fn get_monsters_by_circle(circle: i32) -> Vec<&'static MonsterDef> {
    ALL_MONSTERS.iter().filter(|m| m.circle == circle).copied().collect()
}

pub fn get_monsters_by_region(region: &str) -> Vec<&'static MonsterDef> {
    ALL_MONSTERS.iter().filter(|m| m.region == region).copied().collect()
}

pub fn get_bosses() -> Vec<&'static MonsterDef> {
    ALL_MONSTERS.iter().filter(|m| m.is_boss).copied().collect()
}

// ============================================================
// LOOT SYSTEM
// ============================================================

#[derive(Debug, Clone)]
pub struct LootDropDef {
    pub monster_id: i32,
    pub item_id: i32,
    pub probability: f64,
    pub min_quantity: i32,
    pub max_quantity: i32,
}

pub const MONSTER_DROPS: &[LootDropDef] = &[
    // Circle 1 drops
    LootDropDef { monster_id: 101, item_id: 1, probability: 0.1, min_quantity: 1, max_quantity: 1 },
    LootDropDef { monster_id: 102, item_id: 1, probability: 0.15, min_quantity: 1, max_quantity: 1 },
    LootDropDef { monster_id: 103, item_id: 1, probability: 0.2, min_quantity: 1, max_quantity: 1 },
    LootDropDef { monster_id: 103, item_id: 2, probability: 0.05, min_quantity: 1, max_quantity: 1 },
    LootDropDef { monster_id: 199, item_id: 10, probability: 0.1, min_quantity: 1, max_quantity: 1 }, // Boss drops weapon
];

pub fn get_monster_drops(monster_id: i32) -> Vec<&'static LootDropDef> {
    MONSTER_DROPS.iter().filter(|d| d.monster_id == monster_id).collect()
}

// ============================================================
// SPRITE CONFIG
// ============================================================

#[derive(Debug, Clone)]
pub struct MonsterSpriteConfig {
    pub sprite_type: &'static str,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frames_per_anim: u32,
    pub animation_speed: f32,
}

pub const MONSTER_SPRITES: &[MonsterSpriteConfig] = &[
    // Small (32x32)
    MonsterSpriteConfig { sprite_type: "rat", frame_width: 32, frame_height: 32, frames_per_anim: 4, animation_speed: 0.15 },
    MonsterSpriteConfig { sprite_type: "bat", frame_width: 32, frame_height: 32, frames_per_anim: 4, animation_speed: 0.1 },
    MonsterSpriteConfig { sprite_type: "slime", frame_width: 32, frame_height: 32, frames_per_anim: 4, animation_speed: 0.2 },
    // Medium (48x48)
    MonsterSpriteConfig { sprite_type: "fox", frame_width: 48, frame_height: 48, frames_per_anim: 4, animation_speed: 0.12 },
    MonsterSpriteConfig { sprite_type: "wolf", frame_width: 48, frame_height: 48, frames_per_anim: 4, animation_speed: 0.12 },
    MonsterSpriteConfig { sprite_type: "skeleton", frame_width: 48, frame_height: 48, frames_per_anim: 4, animation_speed: 0.15 },
    MonsterSpriteConfig { sprite_type: "scorpion", frame_width: 48, frame_height: 48, frames_per_anim: 4, animation_speed: 0.12 },
    MonsterSpriteConfig { sprite_type: "mummy", frame_width: 48, frame_height: 48, frames_per_anim: 4, animation_speed: 0.18 },
    MonsterSpriteConfig { sprite_type: "goblin", frame_width: 48, frame_height: 48, frames_per_anim: 4, animation_speed: 0.12 },
    // Large (64x64)
    MonsterSpriteConfig { sprite_type: "golem", frame_width: 64, frame_height: 64, frames_per_anim: 4, animation_speed: 0.2 },
    MonsterSpriteConfig { sprite_type: "ghost", frame_width: 64, frame_height: 64, frames_per_anim: 4, animation_speed: 0.2 },
    MonsterSpriteConfig { sprite_type: "yeti", frame_width: 64, frame_height: 64, frames_per_anim: 4, animation_speed: 0.18 },
    MonsterSpriteConfig { sprite_type: "dark_knight", frame_width: 64, frame_height: 64, frames_per_anim: 4, animation_speed: 0.15 },
    MonsterSpriteConfig { sprite_type: "lich", frame_width: 64, frame_height: 64, frames_per_anim: 4, animation_speed: 0.15 },
    // Boss (128x128)
    MonsterSpriteConfig { sprite_type: "wolf_alpha", frame_width: 128, frame_height: 128, frames_per_anim: 4, animation_speed: 0.15 },
    MonsterSpriteConfig { sprite_type: "scorpion_king", frame_width: 128, frame_height: 128, frames_per_anim: 4, animation_speed: 0.15 },
    MonsterSpriteConfig { sprite_type: "ice_golem", frame_width: 128, frame_height: 128, frames_per_anim: 4, animation_speed: 0.2 },
    MonsterSpriteConfig { sprite_type: "inferno_demon", frame_width: 128, frame_height: 128, frames_per_anim: 4, animation_speed: 0.15 },
    MonsterSpriteConfig { sprite_type: "dark_lord", frame_width: 128, frame_height: 128, frames_per_anim: 4, animation_speed: 0.12 },
];

pub fn get_monster_sprite_config(sprite_type: &str) -> Option<&'static MonsterSpriteConfig> {
    MONSTER_SPRITES.iter().find(|c| c.sprite_type == sprite_type)
}

// ============================================================
// HELPER FUNCTIONS
// ============================================================

pub fn get_monsters_for_level(player_level: i32) -> Vec<&'static MonsterDef> {
    ALL_MONSTERS.iter()
        .filter(|m| (m.level - player_level).abs() <= 5)
        .copied()
        .collect()
}

pub fn calculate_exp_reward(monster: &MonsterDef, player_level: i32) -> i32 {
    let level_diff = monster.level - player_level;
    let multiplier = if level_diff >= 0 {
        (1.0 + (level_diff as f64 * 0.1)).min(1.5)
    } else {
        (1.0 + (level_diff as f64 * 0.1)).max(0.1)
    };
    (monster.exp_reward as f64 * multiplier) as i32
}
