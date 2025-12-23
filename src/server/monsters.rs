//! Monster API handlers

use axum::{extract::Extension, response::Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// MonsterData DTO from database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MonsterDataDto {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub hp_max: i32,
    pub mp_max: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
    pub exp_reward: i32,
    pub gold_min: i32,
    pub gold_max: i32,
    pub sprite_path: Option<String>,
    pub ai_type: Option<String>,
    pub sprite_type: Option<String>,
    pub sprite_size: Option<String>,
}

/// Get all monster definitions from database
pub async fn get_monsters(
    Extension(pool): Extension<PgPool>,
) -> Json<Vec<MonsterDataDto>> {
    let monsters = sqlx::query_as::<_, MonsterDataDto>(
        r#"
        SELECT id, name, level, hp_max, mp_max, attack_min, attack_max, 
               defense, exp_reward, gold_min, gold_max, sprite_path, ai_type,
               sprite_type, sprite_size
        FROM monster_definitions
        ORDER BY level ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(monsters)
}

/// Get monster by ID
pub async fn get_monster_by_id(
    Extension(pool): Extension<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<Option<MonsterDataDto>> {
    let monster = sqlx::query_as::<_, MonsterDataDto>(
        r#"
        SELECT id, name, level, hp_max, mp_max, attack_min, attack_max, 
               defense, exp_reward, gold_min, gold_max, sprite_path, ai_type,
               sprite_type, sprite_size
        FROM monster_definitions
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    Json(monster)
}
