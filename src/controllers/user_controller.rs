use rocket::response::Responder;
use rocket::serde::json::Json;

use crate::{
    entities::{login_request::LoginRequest, user_request::UserRequest},
    models::user::User,
    repositories::user_repository::{add, get_all, login},
};
use crate::entities::user_response::UserResponse;

#[get("/")]
pub async fn get_all_users() -> Json<Vec<User>> {
    let users: Vec<User> = get_all().await;
    Json(users)
}

#[derive(Responder)]
#[response(content_type = "json")]
pub enum AddUserResponse {
    #[response(status = 200)]
    Accepted(UserResponse),
    #[response(status = 400)]
    Rejected(UserResponse),
}

#[post("/", format = "json", data = "<request>")]
pub async fn add_user(request: Json<UserRequest>) -> AddUserResponse {
    match add(&request.0).await {
        Ok(username) => {
            AddUserResponse::Accepted(UserResponse {
                message: format!("User {} successfully created!", username)
            })
        }
        Err(e) => {
            AddUserResponse::Rejected(UserResponse {
                message: e.message()
            })
        }
    }
}

#[derive(Responder)]
#[response(content_type = "json")]
pub enum LoginUserResponse {
    #[response(status = 200)]
    Accepted(UserResponse),
    #[response(status = 404)]
    NotFound(UserResponse),
}

#[post("/login", format = "json", data = "<request>")]
pub async fn login_user(request: Json<LoginRequest>) -> LoginUserResponse {
    match login(request.0).await {
        Ok(username) => {
            LoginUserResponse::Accepted(UserResponse {
                message: format!("User {} successfully logged in!", username)
            })
        }
        Err(e) => {
            LoginUserResponse::NotFound(UserResponse {
                message: e.message()
            })
        }
    }
}
