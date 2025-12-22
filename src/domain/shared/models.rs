use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub vitality: i32,
    pub luck: i32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            vitality: 10,
            luck: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub magic_attack: i32,
    pub magic_defense: i32,
    pub hit_rate: i32,
    pub avoid_rate: i32,
    pub critical_rate: i32,
}

impl CombatStats {
    pub fn from_stats(stats: &Stats, level: i32) -> Self {
        let max_hp = 100 + stats.vitality * 10 + level * 20;
        let max_mp = 50 + stats.intelligence * 5 + level * 10;
        
        Self {
            hp: max_hp,
            max_hp,
            mp: max_mp,
            max_mp,
            attack_min: 5 + stats.strength / 2,
            attack_max: 10 + stats.strength,
            defense: 5 + stats.vitality / 3,
            magic_attack: 5 + stats.intelligence,
            magic_defense: 5 + stats.intelligence / 2,
            hit_rate: 80 + stats.dexterity / 2,
            avoid_rate: 10 + stats.dexterity / 3,
            critical_rate: 5 + stats.luck / 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
