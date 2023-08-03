use rocket::serde::json::Json;

use crate::{models::user::User, repositories::user_repository::get_all};

#[get("/")]
pub async fn get_all_users() -> Json<Vec<User>> {
    let users: Vec<User> = get_all().await;
    Json(users)
}
