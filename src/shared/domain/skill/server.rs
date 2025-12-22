use leptos::prelude::*;
use crate::domain::skill::models::Skill;

#[server(GetSkills, "/api")]
pub async fn get_skills(class: String, level: i32) -> Result<Vec<Skill>, ServerFnError> {
    use axum::extract::State;
    use sqlx::PgPool;
    use leptos_axum::extract;
    
    // In Leptos Axum SSR, we extract the pool from the request via extract()
    // Make sure your main.rs adds PgPool to the layer.
    let pool = extract::<PgPool>().await?;

    let skills = sqlx::query_as::<_, Skill>(
        r#"
        SELECT * FROM skills 
        WHERE (class_req IS NULL OR class_req = $1) 
        AND level_req <= $2
        ORDER BY level_req ASC
        "#
    )
    .bind(class)
    .bind(level)
    .fetch_all(&pool)
    .await?;

    Ok(skills)
}
