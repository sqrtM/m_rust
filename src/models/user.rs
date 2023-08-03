use chrono::Utc;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_on: chrono::DateTime<Utc>,
    pub last_login: chrono::DateTime<Utc>,
}
