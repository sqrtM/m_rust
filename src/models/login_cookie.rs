use chrono::Utc;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct LoginCookie {
    pub cookie_id: i32,
    pub cookie: String,
    pub user_id: i32,
    pub expires_at: chrono::DateTime<Utc>,
    pub created_on: chrono::DateTime<Utc>,
}
