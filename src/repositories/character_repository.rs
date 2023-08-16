use sqlx::{Pool, Postgres};
use crate::db;
use crate::models::character::Character;

pub async fn get_all() -> Vec<Character> {
    let pool: Pool<Postgres> = db().await;
    sqlx::query_as!(Character, "SELECT * FROM characters")
        .fetch_all(&pool)
        .await
        .expect("error with query")
}
