//! Game Resources

use bevy::prelude::*;

/// Game configuration
#[derive(Resource)]
pub struct GameConfig {
    pub tile_size: f32,
    pub player_speed: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tile_size: 32.0,
            player_speed: 150.0,
        }
    }
}

/// Asset handles
#[derive(Resource, Default)]
pub struct GameAssets {
    pub ui_font: Handle<Font>,
    pub assets_loaded: bool,
}

/// Selected character class in character creation
#[derive(Resource, Default)]
pub struct SelectedClass {
    pub class: Option<crate::shared::domain::PlayerClass>,
    pub gender: String,
    pub username: String,
}
