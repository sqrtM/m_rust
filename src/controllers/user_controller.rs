use rocket::serde::json::Json;

use crate::{
    entities::user_request::UserRequest,
    models::user::User,
    repositories::user_repository::{
        add, 
        get_all
    },
};

#[get("/")]
pub async fn get_all_users() -> Json<Vec<User>> {
    let users: Vec<User> = get_all().await;
    Json(users)
}

#[post("/", format = "json", data = "<request>")]
pub async fn add_user(request: Json<UserRequest>) -> Json<User> {
    let created_user: User = add(request.0).await;
    Json(created_user)
}
