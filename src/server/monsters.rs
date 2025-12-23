//! Monster API handlers

use axum::{extract::Extension, response::Json};
use sqlx::PgPool;

use crate::shared::domain::monster::MonsterDataDto;

use crate::shared::domain::monster::LootDrop;

/// Get all monster definitions from database including loot
pub async fn get_monsters(
    Extension(pool): Extension<PgPool>,
) -> Json<Vec<MonsterDataDto>> {
    let mut monsters = sqlx::query_as::<_, MonsterDataDto>(
        r#"

        SELECT id, name, level, hp_max, mp_max, attack_min, attack_max, 
               defense, exp_reward, gold_min, gold_max, sprite_path, ai_type,
               sprite_type, sprite_size, detection_range, attack_range, move_speed,
               description, metadata
        FROM monster_definitions
        ORDER BY level ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Fetch loot for all monsters
    for monster in &mut monsters {
        let loot = sqlx::query_as::<_, LootDrop>(
            "SELECT item_id, probability, min_quantity, max_quantity FROM monster_drops WHERE monster_id = $1"
        )
        .bind(monster.id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
        
        monster.loot = Some(loot);
    }

    Json(monsters)
}

/// Get monster by ID including its loot table
pub async fn get_monster_by_id(
    Extension(pool): Extension<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<Option<MonsterDataDto>> {
    let mut monster = sqlx::query_as::<_, MonsterDataDto>(
        r#"

        SELECT id, name, level, hp_max, mp_max, attack_min, attack_max, 
               defense, exp_reward, gold_min, gold_max, sprite_path, ai_type,
               sprite_type, sprite_size, detection_range, attack_range, move_speed,
               description, metadata
        FROM monster_definitions
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some(ref mut m) = monster {
        let loot = sqlx::query_as::<_, LootDrop>(
            "SELECT item_id, probability, min_quantity, max_quantity FROM monster_drops WHERE monster_id = $1"
        )
        .bind(m.id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
        
        m.loot = Some(loot);
    }

    Json(monster)
}
