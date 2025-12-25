//! Skill data constants - 5 Circle System
//!
//! All skill definitions organized by circle and class.
//! Circle 1: Lv 1-20, Circle 2: Lv 21-40, Circle 3: Lv 41-60, Circle 4: Lv 61-80, Circle 5: Lv 81-99

/// Effect types for skills
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEffectType {
    Damage,
    Heal,
    Buff,
    Debuff,
    DamageOverTime,
    HealOverTime,
}

/// Skill target type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillTarget {
    Single,
    Area,
    Self_,
    Party,
}

/// Skill definition
#[derive(Debug, Clone)]
pub struct SkillDef {
    pub id: i32,
    pub name: &'static str,
    pub name_key: &'static str,
    pub class_id: Option<i32>,  // None = available to all
    pub circle: i32,            // 1-5
    pub req_level: i32,
    pub mp_cost: i32,
    pub cooldown_ms: i32,
    pub description_key: &'static str,
    pub effect_type: SkillEffectType,
    pub target: SkillTarget,
    pub base_value: i32,
    pub icon_path: &'static str,
}

// ============================================================
// WARRIOR SKILLS (Class ID: 1)
// ============================================================

// Circle 1 (Lv 1-20)
pub const BASH: SkillDef = SkillDef {
    id: 101, name: "Bash", name_key: "skill.bash",
    class_id: Some(1), circle: 1, req_level: 1,
    mp_cost: 10, cooldown_ms: 1000,
    description_key: "skill.bash.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 50, icon_path: "/assets/skills/bash.png",
};

pub const CRASH: SkillDef = SkillDef {
    id: 102, name: "Crash", name_key: "skill.crash",
    class_id: Some(1), circle: 1, req_level: 10,
    mp_cost: 30, cooldown_ms: 3000,
    description_key: "skill.crash.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 150, icon_path: "/assets/skills/crash.png",
};

pub const IRON_WILL: SkillDef = SkillDef {
    id: 103, name: "Iron Will", name_key: "skill.iron_will",
    class_id: Some(1), circle: 1, req_level: 20,
    mp_cost: 50, cooldown_ms: 60000,
    description_key: "skill.iron_will.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 20, icon_path: "/assets/skills/iron_will.png",
};

// Circle 2 (Lv 21-40)
pub const WHIRLWIND: SkillDef = SkillDef {
    id: 104, name: "Whirlwind", name_key: "skill.whirlwind",
    class_id: Some(1), circle: 2, req_level: 25,
    mp_cost: 60, cooldown_ms: 5000,
    description_key: "skill.whirlwind.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 200, icon_path: "/assets/skills/whirlwind.png",
};

pub const BATTLE_CRY: SkillDef = SkillDef {
    id: 105, name: "Battle Cry", name_key: "skill.battle_cry",
    class_id: Some(1), circle: 2, req_level: 35,
    mp_cost: 80, cooldown_ms: 120000,
    description_key: "skill.battle_cry.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Party,
    base_value: 30, icon_path: "/assets/skills/battle_cry.png",
};

// Circle 3 (Lv 41-60)
pub const GROUND_SLAM: SkillDef = SkillDef {
    id: 106, name: "Ground Slam", name_key: "skill.ground_slam",
    class_id: Some(1), circle: 3, req_level: 45,
    mp_cost: 100, cooldown_ms: 8000,
    description_key: "skill.ground_slam.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 400, icon_path: "/assets/skills/ground_slam.png",
};

pub const SHIELD_WALL: SkillDef = SkillDef {
    id: 107, name: "Shield Wall", name_key: "skill.shield_wall",
    class_id: Some(1), circle: 3, req_level: 55,
    mp_cost: 120, cooldown_ms: 180000,
    description_key: "skill.shield_wall.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 50, icon_path: "/assets/skills/shield_wall.png",
};

// Circle 4 (Lv 61-80)
pub const BERSERK: SkillDef = SkillDef {
    id: 108, name: "Berserk", name_key: "skill.berserk",
    class_id: Some(1), circle: 4, req_level: 65,
    mp_cost: 150, cooldown_ms: 300000,
    description_key: "skill.berserk.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 100, icon_path: "/assets/skills/berserk.png",
};

pub const EARTHQUAKE: SkillDef = SkillDef {
    id: 109, name: "Earthquake", name_key: "skill.earthquake",
    class_id: Some(1), circle: 4, req_level: 75,
    mp_cost: 200, cooldown_ms: 15000,
    description_key: "skill.earthquake.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 800, icon_path: "/assets/skills/earthquake.png",
};

// Circle 5 (Lv 81-99)
pub const TITAN_STRIKE: SkillDef = SkillDef {
    id: 110, name: "Titan Strike", name_key: "skill.titan_strike",
    class_id: Some(1), circle: 5, req_level: 85,
    mp_cost: 300, cooldown_ms: 20000,
    description_key: "skill.titan_strike.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 2000, icon_path: "/assets/skills/titan_strike.png",
};

pub const IMMORTAL: SkillDef = SkillDef {
    id: 111, name: "Immortal", name_key: "skill.immortal",
    class_id: Some(1), circle: 5, req_level: 95,
    mp_cost: 500, cooldown_ms: 600000,
    description_key: "skill.immortal.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 100, icon_path: "/assets/skills/immortal.png",
};

// ============================================================
// ROGUE SKILLS (Class ID: 2)
// ============================================================

// Circle 1
pub const DOUBLE_STAB: SkillDef = SkillDef {
    id: 201, name: "Double Stab", name_key: "skill.double_stab",
    class_id: Some(2), circle: 1, req_level: 1,
    mp_cost: 10, cooldown_ms: 1000,
    description_key: "skill.double_stab.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 40, icon_path: "/assets/skills/double_stab.png",
};

pub const AMBUSH: SkillDef = SkillDef {
    id: 202, name: "Ambush", name_key: "skill.ambush",
    class_id: Some(2), circle: 1, req_level: 10,
    mp_cost: 30, cooldown_ms: 5000,
    description_key: "skill.ambush.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 180, icon_path: "/assets/skills/ambush.png",
};

pub const EVASION: SkillDef = SkillDef {
    id: 203, name: "Evasion", name_key: "skill.evasion",
    class_id: Some(2), circle: 1, req_level: 20,
    mp_cost: 40, cooldown_ms: 30000,
    description_key: "skill.evasion.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 50, icon_path: "/assets/skills/evasion.png",
};

// Circle 2
pub const POISON_BLADE: SkillDef = SkillDef {
    id: 204, name: "Poison Blade", name_key: "skill.poison_blade",
    class_id: Some(2), circle: 2, req_level: 25,
    mp_cost: 50, cooldown_ms: 3000,
    description_key: "skill.poison_blade.desc",
    effect_type: SkillEffectType::DamageOverTime, target: SkillTarget::Single,
    base_value: 100, icon_path: "/assets/skills/poison_blade.png",
};

pub const SHADOW_STEP: SkillDef = SkillDef {
    id: 205, name: "Shadow Step", name_key: "skill.shadow_step",
    class_id: Some(2), circle: 2, req_level: 35,
    mp_cost: 60, cooldown_ms: 10000,
    description_key: "skill.shadow_step.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 0, icon_path: "/assets/skills/shadow_step.png",
};

// Circle 3
pub const FAN_OF_KNIVES: SkillDef = SkillDef {
    id: 206, name: "Fan of Knives", name_key: "skill.fan_of_knives",
    class_id: Some(2), circle: 3, req_level: 45,
    mp_cost: 80, cooldown_ms: 6000,
    description_key: "skill.fan_of_knives.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 300, icon_path: "/assets/skills/fan_of_knives.png",
};

pub const VANISH: SkillDef = SkillDef {
    id: 207, name: "Vanish", name_key: "skill.vanish",
    class_id: Some(2), circle: 3, req_level: 55,
    mp_cost: 100, cooldown_ms: 60000,
    description_key: "skill.vanish.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 100, icon_path: "/assets/skills/vanish.png",
};

// Circle 4
pub const DEADLY_POISON: SkillDef = SkillDef {
    id: 208, name: "Deadly Poison", name_key: "skill.deadly_poison",
    class_id: Some(2), circle: 4, req_level: 65,
    mp_cost: 120, cooldown_ms: 5000,
    description_key: "skill.deadly_poison.desc",
    effect_type: SkillEffectType::DamageOverTime, target: SkillTarget::Single,
    base_value: 500, icon_path: "/assets/skills/deadly_poison.png",
};

pub const SHADOW_DANCE: SkillDef = SkillDef {
    id: 209, name: "Shadow Dance", name_key: "skill.shadow_dance",
    class_id: Some(2), circle: 4, req_level: 75,
    mp_cost: 150, cooldown_ms: 180000,
    description_key: "skill.shadow_dance.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 100, icon_path: "/assets/skills/shadow_dance.png",
};

// Circle 5
pub const ASSASSINATE: SkillDef = SkillDef {
    id: 210, name: "Assassinate", name_key: "skill.assassinate",
    class_id: Some(2), circle: 5, req_level: 85,
    mp_cost: 250, cooldown_ms: 30000,
    description_key: "skill.assassinate.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 3000, icon_path: "/assets/skills/assassinate.png",
};

pub const DEATH_MARK: SkillDef = SkillDef {
    id: 211, name: "Death Mark", name_key: "skill.death_mark",
    class_id: Some(2), circle: 5, req_level: 95,
    mp_cost: 400, cooldown_ms: 120000,
    description_key: "skill.death_mark.desc",
    effect_type: SkillEffectType::Debuff, target: SkillTarget::Single,
    base_value: 200, icon_path: "/assets/skills/death_mark.png",
};

// ============================================================
// MAGE SKILLS (Class ID: 3)
// ============================================================

// Circle 1
pub const FIREBALL: SkillDef = SkillDef {
    id: 301, name: "Fireball", name_key: "skill.fireball",
    class_id: Some(3), circle: 1, req_level: 1,
    mp_cost: 15, cooldown_ms: 1500,
    description_key: "skill.fireball.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 60, icon_path: "/assets/skills/fireball.png",
};

pub const THUNDER_BOLT: SkillDef = SkillDef {
    id: 302, name: "Thunder Bolt", name_key: "skill.thunder_bolt",
    class_id: Some(3), circle: 1, req_level: 10,
    mp_cost: 40, cooldown_ms: 4000,
    description_key: "skill.thunder_bolt.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 160, icon_path: "/assets/skills/thunder_bolt.png",
};

pub const ICE_SHIELD: SkillDef = SkillDef {
    id: 303, name: "Ice Shield", name_key: "skill.ice_shield",
    class_id: Some(3), circle: 1, req_level: 20,
    mp_cost: 60, cooldown_ms: 45000,
    description_key: "skill.ice_shield.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 50, icon_path: "/assets/skills/ice_shield.png",
};

// Circle 2
pub const FLAME_WAVE: SkillDef = SkillDef {
    id: 304, name: "Flame Wave", name_key: "skill.flame_wave",
    class_id: Some(3), circle: 2, req_level: 25,
    mp_cost: 70, cooldown_ms: 5000,
    description_key: "skill.flame_wave.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 250, icon_path: "/assets/skills/flame_wave.png",
};

pub const TELEPORT: SkillDef = SkillDef {
    id: 305, name: "Teleport", name_key: "skill.teleport",
    class_id: Some(3), circle: 2, req_level: 35,
    mp_cost: 80, cooldown_ms: 20000,
    description_key: "skill.teleport.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 0, icon_path: "/assets/skills/teleport.png",
};

// Circle 3
pub const BLIZZARD: SkillDef = SkillDef {
    id: 306, name: "Blizzard", name_key: "skill.blizzard",
    class_id: Some(3), circle: 3, req_level: 45,
    mp_cost: 100, cooldown_ms: 8000,
    description_key: "skill.blizzard.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 400, icon_path: "/assets/skills/blizzard.png",
};

pub const MANA_SHIELD: SkillDef = SkillDef {
    id: 307, name: "Mana Shield", name_key: "skill.mana_shield",
    class_id: Some(3), circle: 3, req_level: 55,
    mp_cost: 200, cooldown_ms: 120000,
    description_key: "skill.mana_shield.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 100, icon_path: "/assets/skills/mana_shield.png",
};

// Circle 4
pub const METEOR: SkillDef = SkillDef {
    id: 308, name: "Meteor", name_key: "skill.meteor",
    class_id: Some(3), circle: 4, req_level: 65,
    mp_cost: 180, cooldown_ms: 15000,
    description_key: "skill.meteor.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 1000, icon_path: "/assets/skills/meteor.png",
};

pub const TIME_STOP: SkillDef = SkillDef {
    id: 309, name: "Time Stop", name_key: "skill.time_stop",
    class_id: Some(3), circle: 4, req_level: 75,
    mp_cost: 250, cooldown_ms: 300000,
    description_key: "skill.time_stop.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 0, icon_path: "/assets/skills/time_stop.png",
};

// Circle 5
pub const ARMAGEDDON: SkillDef = SkillDef {
    id: 310, name: "Armageddon", name_key: "skill.armageddon",
    class_id: Some(3), circle: 5, req_level: 85,
    mp_cost: 400, cooldown_ms: 30000,
    description_key: "skill.armageddon.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 2500, icon_path: "/assets/skills/armageddon.png",
};

pub const ARCANE_MASTERY: SkillDef = SkillDef {
    id: 311, name: "Arcane Mastery", name_key: "skill.arcane_mastery",
    class_id: Some(3), circle: 5, req_level: 95,
    mp_cost: 500, cooldown_ms: 600000,
    description_key: "skill.arcane_mastery.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 200, icon_path: "/assets/skills/arcane_mastery.png",
};

// ============================================================
// CLERIC SKILLS (Class ID: 4)
// ============================================================

// Circle 1
pub const HEAL: SkillDef = SkillDef {
    id: 401, name: "Heal", name_key: "skill.heal",
    class_id: Some(4), circle: 1, req_level: 1,
    mp_cost: 20, cooldown_ms: 2000,
    description_key: "skill.heal.desc",
    effect_type: SkillEffectType::Heal, target: SkillTarget::Single,
    base_value: 40, icon_path: "/assets/skills/heal.png",
};

pub const HOLY_BOLT: SkillDef = SkillDef {
    id: 402, name: "Holy Bolt", name_key: "skill.holy_bolt",
    class_id: Some(4), circle: 1, req_level: 5,
    mp_cost: 25, cooldown_ms: 2000,
    description_key: "skill.holy_bolt.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 50, icon_path: "/assets/skills/holy_bolt.png",
};

pub const GREAT_HEAL: SkillDef = SkillDef {
    id: 403, name: "Great Heal", name_key: "skill.great_heal",
    class_id: Some(4), circle: 1, req_level: 20,
    mp_cost: 60, cooldown_ms: 5000,
    description_key: "skill.great_heal.desc",
    effect_type: SkillEffectType::Heal, target: SkillTarget::Single,
    base_value: 150, icon_path: "/assets/skills/great_heal.png",
};

// Circle 2
pub const BLESSING: SkillDef = SkillDef {
    id: 404, name: "Blessing", name_key: "skill.blessing",
    class_id: Some(4), circle: 2, req_level: 25,
    mp_cost: 80, cooldown_ms: 120000,
    description_key: "skill.blessing.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Party,
    base_value: 20, icon_path: "/assets/skills/blessing.png",
};

pub const SANCTUARY: SkillDef = SkillDef {
    id: 405, name: "Sanctuary", name_key: "skill.sanctuary",
    class_id: Some(4), circle: 2, req_level: 35,
    mp_cost: 100, cooldown_ms: 60000,
    description_key: "skill.sanctuary.desc",
    effect_type: SkillEffectType::HealOverTime, target: SkillTarget::Area,
    base_value: 50, icon_path: "/assets/skills/sanctuary.png",
};

// Circle 3
pub const MASS_HEAL: SkillDef = SkillDef {
    id: 406, name: "Mass Heal", name_key: "skill.mass_heal",
    class_id: Some(4), circle: 3, req_level: 45,
    mp_cost: 150, cooldown_ms: 8000,
    description_key: "skill.mass_heal.desc",
    effect_type: SkillEffectType::Heal, target: SkillTarget::Party,
    base_value: 300, icon_path: "/assets/skills/mass_heal.png",
};

pub const HOLY_ARMOR: SkillDef = SkillDef {
    id: 407, name: "Holy Armor", name_key: "skill.holy_armor",
    class_id: Some(4), circle: 3, req_level: 55,
    mp_cost: 120, cooldown_ms: 180000,
    description_key: "skill.holy_armor.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Party,
    base_value: 50, icon_path: "/assets/skills/holy_armor.png",
};

// Circle 4
pub const RESURRECTION: SkillDef = SkillDef {
    id: 408, name: "Resurrection", name_key: "skill.resurrection",
    class_id: Some(4), circle: 4, req_level: 65,
    mp_cost: 300, cooldown_ms: 600000,
    description_key: "skill.resurrection.desc",
    effect_type: SkillEffectType::Heal, target: SkillTarget::Single,
    base_value: 100, icon_path: "/assets/skills/resurrection.png",
};

pub const DIVINE_JUDGMENT: SkillDef = SkillDef {
    id: 409, name: "Divine Judgment", name_key: "skill.divine_judgment",
    class_id: Some(4), circle: 4, req_level: 75,
    mp_cost: 200, cooldown_ms: 15000,
    description_key: "skill.divine_judgment.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 600, icon_path: "/assets/skills/divine_judgment.png",
};

// Circle 5
pub const GUARDIAN_ANGEL: SkillDef = SkillDef {
    id: 410, name: "Guardian Angel", name_key: "skill.guardian_angel",
    class_id: Some(4), circle: 5, req_level: 85,
    mp_cost: 400, cooldown_ms: 300000,
    description_key: "skill.guardian_angel.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Party,
    base_value: 100, icon_path: "/assets/skills/guardian_angel.png",
};

pub const DIVINE_INTERVENTION: SkillDef = SkillDef {
    id: 411, name: "Divine Intervention", name_key: "skill.divine_intervention",
    class_id: Some(4), circle: 5, req_level: 95,
    mp_cost: 500, cooldown_ms: 900000,
    description_key: "skill.divine_intervention.desc",
    effect_type: SkillEffectType::Heal, target: SkillTarget::Party,
    base_value: 100, icon_path: "/assets/skills/divine_intervention.png",
};

// ============================================================
// MARTIAL ARTIST SKILLS (Class ID: 5)
// ============================================================

// Circle 1
pub const PUNCH: SkillDef = SkillDef {
    id: 501, name: "Punch", name_key: "skill.punch",
    class_id: Some(5), circle: 1, req_level: 1,
    mp_cost: 5, cooldown_ms: 500,
    description_key: "skill.punch.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 30, icon_path: "/assets/skills/punch.png",
};

pub const POWER_KICK: SkillDef = SkillDef {
    id: 502, name: "Power Kick", name_key: "skill.power_kick",
    class_id: Some(5), circle: 1, req_level: 10,
    mp_cost: 20, cooldown_ms: 2000,
    description_key: "skill.power_kick.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 80, icon_path: "/assets/skills/power_kick.png",
};

pub const INNER_PEACE: SkillDef = SkillDef {
    id: 503, name: "Inner Peace", name_key: "skill.inner_peace",
    class_id: Some(5), circle: 1, req_level: 20,
    mp_cost: 30, cooldown_ms: 30000,
    description_key: "skill.inner_peace.desc",
    effect_type: SkillEffectType::HealOverTime, target: SkillTarget::Self_,
    base_value: 30, icon_path: "/assets/skills/inner_peace.png",
};

// Circle 2
pub const DRAGON_FIST: SkillDef = SkillDef {
    id: 504, name: "Dragon Fist", name_key: "skill.dragon_fist",
    class_id: Some(5), circle: 2, req_level: 25,
    mp_cost: 40, cooldown_ms: 3000,
    description_key: "skill.dragon_fist.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 200, icon_path: "/assets/skills/dragon_fist.png",
};

pub const IRON_BODY: SkillDef = SkillDef {
    id: 505, name: "Iron Body", name_key: "skill.iron_body",
    class_id: Some(5), circle: 2, req_level: 35,
    mp_cost: 60, cooldown_ms: 60000,
    description_key: "skill.iron_body.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 50, icon_path: "/assets/skills/iron_body.png",
};

// Circle 3
pub const TIGER_PALM: SkillDef = SkillDef {
    id: 506, name: "Tiger Palm", name_key: "skill.tiger_palm",
    class_id: Some(5), circle: 3, req_level: 45,
    mp_cost: 70, cooldown_ms: 4000,
    description_key: "skill.tiger_palm.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 350, icon_path: "/assets/skills/tiger_palm.png",
};

pub const FLYING_KICK: SkillDef = SkillDef {
    id: 507, name: "Flying Kick", name_key: "skill.flying_kick",
    class_id: Some(5), circle: 3, req_level: 55,
    mp_cost: 80, cooldown_ms: 6000,
    description_key: "skill.flying_kick.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Area,
    base_value: 400, icon_path: "/assets/skills/flying_kick.png",
};

// Circle 4
pub const PRESSURE_POINT: SkillDef = SkillDef {
    id: 508, name: "Pressure Point", name_key: "skill.pressure_point",
    class_id: Some(5), circle: 4, req_level: 65,
    mp_cost: 100, cooldown_ms: 8000,
    description_key: "skill.pressure_point.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 600, icon_path: "/assets/skills/pressure_point.png",
};

pub const CHI_BURST: SkillDef = SkillDef {
    id: 509, name: "Chi Burst", name_key: "skill.chi_burst",
    class_id: Some(5), circle: 4, req_level: 75,
    mp_cost: 150, cooldown_ms: 120000,
    description_key: "skill.chi_burst.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 100, icon_path: "/assets/skills/chi_burst.png",
};

// Circle 5
pub const HUNDRED_FISTS: SkillDef = SkillDef {
    id: 510, name: "Hundred Fists", name_key: "skill.hundred_fists",
    class_id: Some(5), circle: 5, req_level: 85,
    mp_cost: 200, cooldown_ms: 20000,
    description_key: "skill.hundred_fists.desc",
    effect_type: SkillEffectType::Damage, target: SkillTarget::Single,
    base_value: 2000, icon_path: "/assets/skills/hundred_fists.png",
};

pub const ENLIGHTENMENT: SkillDef = SkillDef {
    id: 511, name: "Enlightenment", name_key: "skill.enlightenment",
    class_id: Some(5), circle: 5, req_level: 95,
    mp_cost: 300, cooldown_ms: 600000,
    description_key: "skill.enlightenment.desc",
    effect_type: SkillEffectType::Buff, target: SkillTarget::Self_,
    base_value: 200, icon_path: "/assets/skills/enlightenment.png",
};

// ============================================================
// SKILL REGISTRY
// ============================================================

/// All skill definitions
pub const ALL_SKILLS: &[&SkillDef] = &[
    // Warrior
    &BASH, &CRASH, &IRON_WILL, &WHIRLWIND, &BATTLE_CRY,
    &GROUND_SLAM, &SHIELD_WALL, &BERSERK, &EARTHQUAKE, &TITAN_STRIKE, &IMMORTAL,
    // Rogue
    &DOUBLE_STAB, &AMBUSH, &EVASION, &POISON_BLADE, &SHADOW_STEP,
    &FAN_OF_KNIVES, &VANISH, &DEADLY_POISON, &SHADOW_DANCE, &ASSASSINATE, &DEATH_MARK,
    // Mage
    &FIREBALL, &THUNDER_BOLT, &ICE_SHIELD, &FLAME_WAVE, &TELEPORT,
    &BLIZZARD, &MANA_SHIELD, &METEOR, &TIME_STOP, &ARMAGEDDON, &ARCANE_MASTERY,
    // Cleric
    &HEAL, &HOLY_BOLT, &GREAT_HEAL, &BLESSING, &SANCTUARY,
    &MASS_HEAL, &HOLY_ARMOR, &RESURRECTION, &DIVINE_JUDGMENT, &GUARDIAN_ANGEL, &DIVINE_INTERVENTION,
    // Martial Artist
    &PUNCH, &POWER_KICK, &INNER_PEACE, &DRAGON_FIST, &IRON_BODY,
    &TIGER_PALM, &FLYING_KICK, &PRESSURE_POINT, &CHI_BURST, &HUNDRED_FISTS, &ENLIGHTENMENT,
];

/// Get skill by ID
pub fn get_skill_by_id(id: i32) -> Option<&'static SkillDef> {
    ALL_SKILLS.iter().find(|s| s.id == id).copied()
}

/// Get skills for a specific class
pub fn get_skills_for_class(class_id: i32) -> Vec<&'static SkillDef> {
    ALL_SKILLS.iter()
        .filter(|s| s.class_id == Some(class_id) || s.class_id.is_none())
        .copied()
        .collect()
}

/// Get skills for a specific class and circle
pub fn get_skills_for_class_circle(class_id: i32, circle: i32) -> Vec<&'static SkillDef> {
    ALL_SKILLS.iter()
        .filter(|s| (s.class_id == Some(class_id) || s.class_id.is_none()) && s.circle == circle)
        .copied()
        .collect()
}

/// Get available skills for a class at a given level
pub fn get_available_skills(class_id: i32, level: i32) -> Vec<&'static SkillDef> {
    ALL_SKILLS.iter()
        .filter(|s| (s.class_id == Some(class_id) || s.class_id.is_none()) && s.req_level <= level)
        .copied()
        .collect()
}
