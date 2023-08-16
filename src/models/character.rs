use chrono::Utc;
use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Character {
    pub character_id: i32,
    pub user_id: i32,
    pub character_name: String,
    pub constitution: i32,
    pub strength: i32,
    pub madness: i32,
    pub intelligence: i32,
    pub created_at: chrono::DateTime<Utc>,
}
