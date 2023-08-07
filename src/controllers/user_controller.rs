use rocket::serde::json::Json;

use crate::{
    models::user::User,
    repositories::user_repository::{add, get_all, login},
};
use rocket::http::CookieJar;

use crate::controllers::{ResponseStatus, set_login_cookie};
use crate::entities::authorization::api_auth_error::{ApiKey, ApiAuthStatus};
use crate::entities::Construct;
use crate::entities::user::login_request::LoginRequest;
use crate::entities::user::user_request::UserRequest;
use crate::entities::user::user_response::UserResponse;

#[get("/")]
pub async fn get_all_users(key: Result<ApiKey<'_>, ApiAuthStatus>) -> ResponseStatus<Json<Vec<User>>> {
    match key {
        Ok(_) => {
            let users: Vec<User> = get_all().await;
            ResponseStatus::Accepted(Json(users))
        },
        Err(_) => ResponseStatus::Unauthorized(Json(vec![])),
    }
}

#[post("/", format = "json", data = "<request>")]
pub async fn add_user(request: Json<UserRequest>, jar: &CookieJar<'_>) -> ResponseStatus<Json<UserResponse>> {
    match add(&request.0).await {
        Ok(username) => {
            set_login_cookie(&username, jar);
            ResponseStatus::Accepted(Json(UserResponse {
                message: format!("User {} successfully created!", username)
            }))
        }
        Err(e) => e.construct()
    }
}

#[post("/login", format = "json", data = "<request>")]
pub async fn login_user(request: Json<LoginRequest>, jar: &CookieJar<'_>) -> ResponseStatus<Json<UserResponse>> {
    match login(request.0).await {
        Ok(username) => {
            set_login_cookie(&username, jar);
            ResponseStatus::Accepted(Json(UserResponse {
                message: format!("User {} successfully logged in!", username)
            }))
        }
        Err(e) => e.construct()
    }
}