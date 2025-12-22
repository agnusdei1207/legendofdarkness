use leptos::prelude::*;
use crate::shared::domain::player::Player;

use leptos::prelude::*;
use crate::shared::domain::player::Player;

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<Option<Player>, ServerFnError> {
    use crate::server::db::get_db_pool;
    use bcrypt::verify;
    
    let pool = get_db_pool();
    
    // 1. Get User
    let row = sqlx::query!("SELECT id, password_hash FROM users WHERE username = $1", username)
        .fetch_optional(pool)
        .await?;
        
    let user = match row {
        Some(u) => u,
        None => return Ok(None),
    };
    
    // 2. Verify Password
    if !verify(&password, &user.password_hash).unwrap_or(false) {
        return Ok(None);
    }
    
    // 3. Get Character (Assuming single character for now for simplicity, or select first)
    // We need to map DB columns to Player struct.
    // This is complex because Player struct has nested Stats, etc.
    // For now, let's fetch basic info and construct Player.
    // Ideally we use sqlx::query_as! but types need to match strictly.
    
    // Let's implement a simplified fetch.
    // Note: The schema for `characters` has basic fields. `character_stats` has detailed stats.
    
    let char_row = sqlx::query!(
        r#"
        SELECT c.*, s.str, s.dex, s.con, s.int_stat, s.wis, s.stat_points_available
        FROM characters c
        LEFT JOIN character_stats s ON c.id = s.character_id
        WHERE c.user_id = $1
        LIMIT 1
        "#,
        user.id
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some(c) = char_row {
        // Map to Player Model
        // We need to convert String map/class to Enums.
        let player_class = match c.class_id {
            Some(1) => crate::shared::domain::character::models::PlayerClass::Warrior,
            Some(2) => crate::shared::domain::character::models::PlayerClass::Rogue,
            Some(3) => crate::shared::domain::character::models::PlayerClass::Mage,
            Some(4) => crate::shared::domain::character::models::PlayerClass::Cleric,
            Some(5) => crate::shared::domain::character::models::PlayerClass::MartialArtist,
            _ => crate::shared::domain::character::models::PlayerClass::Warrior,
        };
        
        // Construct stats
        let stats = crate::shared::domain::shared::models::Stats {
            str: c.str.unwrap_or(10),
            dex: c.dex.unwrap_or(10),
            con: c.con.unwrap_or(10),
            int: c.int_stat.unwrap_or(10), // int_stat column name
            wis: c.wis.unwrap_or(10),
        };
        
        let combat_stats = crate::shared::domain::shared::models::CombatStats::from_stats(&stats, c.level.unwrap_or(1));

        let p = Player {
            id: c.id.to_string(),
            username: c.name,
            gender: c.gender,
            class: player_class,
            level: c.level.unwrap_or(1),
            exp: c.exp.unwrap_or(0),
            exp_to_next_level: 100, // Calc logic needed
            stats,
            stat_points: c.stat_points_available.unwrap_or(0),
            combat_stats,
            equipment: std::collections::HashMap::new(), // Load from DB needed
            inventory: vec![None; 24], // Load from DB needed
            current_map: c.map_id,
            position: crate::shared::domain::shared::models::Position { 
                x: c.x.unwrap_or(400) as f64, 
                y: c.y.unwrap_or(300) as f64 
            },
            direction: crate::shared::domain::shared::models::Direction::Down,
            gold: c.gold.unwrap_or(0),
            is_moving: false,
            is_attacking: false,
            target_monster_id: None,
        };
        
        return Ok(Some(p));
    }
    
    Ok(None)
}

#[server(Register, "/api")]
pub async fn register(username: String, password: String, class_idx: i32) -> Result<String, ServerFnError> {
    use crate::server::db::get_db_pool;
    use bcrypt::{hash, DEFAULT_COST};
    use uuid::Uuid;
    
    let pool = get_db_pool();
    
    // Hash password
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash");
    
    // Create User
    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)",
        user_id, username, hashed
    )
    .execute(pool)
    .await?;
    
    // Create Character (Default)
    let char_id = Uuid::new_v4();
    let class_id = class_idx; // 1..5
    
    // Basic stats based on class
    // For simplicity, inserting default 10s. Normally depend on class.
    
    sqlx::query!(
        "INSERT INTO characters (id, user_id, name, class_id, gender) VALUES ($1, $2, $3, $4, 'male')",
        char_id, user_id, username, class_id
    )
    .execute(pool)
    .await?;
    
    sqlx::query!(
        "INSERT INTO character_stats (character_id, str, dex, con, int_stat, wis) VALUES ($1, 10, 10, 10, 10, 10)",
        char_id
    )
    .execute(pool)
    .await?;
    
    Ok("Success".to_string())
}
