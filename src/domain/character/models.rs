use serde::{Deserialize, Serialize};
use crate::domain::shared::models::{Position, Stats, CombatStats, Direction};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub usersname: String,
    pub gender: String, // 'male' or 'female'
    pub class: PlayerClass,
    pub level: i32,
    pub exp: i64,
    pub exp_to_next_level: i64,
    
    // 스탯
    pub stats: Stats,
    pub stat_points: i32,
    
    // 전투 스탯
    pub combat_stats: CombatStats,
    
    // 위치
    pub current_map: String,
    pub position: Position,
    pub direction: Direction,
    
    // 재화
    pub gold: i64,
    
    // 상태
    pub is_moving: bool,
    pub is_attacking: bool,
    pub target_monster_id: Option<String>,
}

impl Player {
    pub fn new(username: String, class: PlayerClass) -> Self {
        let stats = class.get_base_stats();
        let combat_stats = CombatStats::from_stats(&stats, 1);
        
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            gender: "male".to_string(),
            class,
            level: 1,
            exp: 0,
            exp_to_next_level: 100,
            stats,
            stat_points: 0,
            combat_stats,
            current_map: "village".to_string(), // Updated to match DB seed
            position: Position::new(400.0, 300.0),
            direction: Direction::Down,
            gold: 100,
            is_moving: false,
            is_attacking: false,
            target_monster_id: None,
        }
    }
    
    pub fn add_exp(&mut self, amount: i64) {
        self.exp += amount;
        while self.exp >= self.exp_to_next_level {
            self.level_up();
        }
    }
    
    fn level_up(&mut self) {
        self.exp -= self.exp_to_next_level;
        self.level += 1;
        self.exp_to_next_level = self.calculate_exp_to_next_level();
        self.stat_points += 5;
        
        // 레벨업 시 HP/MP 회복
        self.combat_stats = CombatStats::from_stats(&self.stats, self.level);
    }
    
    fn calculate_exp_to_next_level(&self) -> i64 {
        (100.0 * (self.level as f64).powf(1.5)) as i64
    }
    
    pub fn add_stat(&mut self, stat_type: StatType, amount: i32) {
        if self.stat_points >= amount {
            match stat_type {
                StatType::Strength => self.stats.strength += amount,
                StatType::Dexterity => self.stats.dexterity += amount,
                StatType::Intelligence => self.stats.intelligence += amount,
                StatType::Vitality => self.stats.vitality += amount,
                StatType::Luck => self.stats.luck += amount,
            }
            self.stat_points -= amount;
            self.combat_stats = CombatStats::from_stats(&self.stats, self.level);
        }
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        self.combat_stats.hp = (self.combat_stats.hp - damage).max(0);
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.combat_stats.hp = (self.combat_stats.hp + amount).min(self.combat_stats.max_hp);
    }
    
    pub fn is_dead(&self) -> bool {
        self.combat_stats.hp <= 0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PlayerClass {
    Warrior,   // 전사
    Archer,    // 궁수
    Mage,      // 마법사
    Rogue,     // 도적
}

impl PlayerClass {
    pub fn name(&self) -> &str {
        match self {
            PlayerClass::Warrior => "전사",
            PlayerClass::Archer => "궁수",
            PlayerClass::Mage => "마법사",
            PlayerClass::Rogue => "도적",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            PlayerClass::Warrior => "강력한 물리 공격력과 높은 체력을 가진 근접 전투의 달인",
            PlayerClass::Archer => "빠른 공격 속도와 원거리 공격이 특기인 민첩한 전사",
            PlayerClass::Mage => "강력한 마법으로 적을 제압하는 지혜로운 전사",
            PlayerClass::Rogue => "빠른 속도와 치명타로 적을 암살하는 그림자의 전사",
        }
    }
    
    pub fn get_base_stats(&self) -> Stats {
        match self {
            PlayerClass::Warrior => Stats {
                strength: 20,
                dexterity: 10,
                intelligence: 5,
                vitality: 18,
                luck: 7,
            },
            PlayerClass::Archer => Stats {
                strength: 12,
                dexterity: 20,
                intelligence: 8,
                vitality: 12,
                luck: 15,
            },
            PlayerClass::Mage => Stats {
                strength: 5,
                dexterity: 8,
                intelligence: 25,
                vitality: 10,
                luck: 12,
            },
            PlayerClass::Rogue => Stats {
                strength: 10,
                dexterity: 18,
                intelligence: 10,
                vitality: 10,
                luck: 20,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatType {
    Strength,
    Dexterity,
    Intelligence,
    Vitality,
    Luck,
}
