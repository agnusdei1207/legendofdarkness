use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMap {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub width: usize,
    pub height: usize,
    pub tile_size: usize,
    pub min_level: i32,
    pub pvp_enabled: bool,
}

impl GameMap {
    pub fn get_pixel_width(&self) -> f64 {
        (self.width * self.tile_size) as f64
    }
    
    pub fn get_pixel_height(&self) -> f64 {
        (self.height * self.tile_size) as f64
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TileType {
    Grass,
    Stone,
    Water,
    Wall,
    Portal,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Grass | TileType::Stone | TileType::Portal => true,
            TileType::Water | TileType::Wall => false,
        }
    }
    
    pub fn color(&self) -> &str {
        match self {
            TileType::Grass => "#228b22",
            TileType::Stone => "#808080",
            TileType::Water => "#1e90ff",
            TileType::Wall => "#654321",
            TileType::Portal => "#ff00ff",
        }
    }
}
