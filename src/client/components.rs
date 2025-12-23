//! Game Components (ECS)

use bevy::prelude::*;
use crate::shared::domain::{Direction, PlayerClass, MonsterAIType};
use crate::shared::domain::character::models::Player;
use crate::shared::domain::monster::Monster;

// ============ Player Components ============

/// Marker component for the player entity
#[derive(Component)]
pub struct PlayerComponent;

// ============ Monster Components ============

/// Marker component for monster entities
#[derive(Component)]
pub struct MonsterComponent;

/// Monster AI behavior
#[derive(Component)]
pub struct MonsterAI {
    pub ai_type: MonsterAIType,
    pub detection_range: f32,
    pub attack_range: f32,
    pub move_speed: f32,
    pub target: Option<Entity>,
    pub spawn_position: Vec2,
}

// ============ Common Components ============

/// Movement/Direction component
#[derive(Component)]
pub struct Facing {
    pub direction: Direction,
}

impl Default for Facing {
    fn default() -> Self {
        Self {
            direction: Direction::Down,
        }
    }
}

/// Velocity component for movement
#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Animation component
#[derive(Component)]
pub struct AnimationState {
    pub current_frame: usize,
    pub frame_count: usize,
    pub timer: Timer,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_frame: 0,
            frame_count: 4,
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
        }
    }
}

/// Combat state component
#[derive(Component)]
pub struct CombatState {
    pub is_attacking: bool,
    pub attack_timer: Timer,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            is_attacking: false,
            attack_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

// ============ UI Components ============

/// Marker for main menu UI
#[derive(Component)]
pub struct MainMenuUI;

/// Marker for character select UI
#[derive(Component)]
pub struct CharacterSelectUI;

/// Marker for HUD elements
#[derive(Component)]
pub struct HudUI;

/// Button action types
#[derive(Component, Clone)]
pub enum ButtonAction {
    CharacterSelect,
    SelectClass(PlayerClass),
    ConfirmCharacter,
    BackToMenu,
    Quit,
}

/// HP Bar UI
#[derive(Component)]
pub struct HpBar;

/// MP Bar UI
#[derive(Component)]
pub struct MpBar;

/// Exp Bar UI  
#[derive(Component)]
pub struct ExpBar;

/// Level text
#[derive(Component)]
pub struct LevelText;

/// Gold text
#[derive(Component)]
pub struct GoldText;

// ============ World Components ============

/// Tile component
#[derive(Component)]
pub struct Tile;

/// Camera follow target
#[derive(Component)]
pub struct CameraTarget;
