//! Monster API handlers

use axum::{extract::Extension, response::Json};
use sqlx::PgPool;

use crate::shared::domain::monster::MonsterDataDto;

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
