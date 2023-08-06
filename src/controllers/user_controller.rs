use rocket::serde::json::Json;

use crate::{
    entities::{login_request::LoginRequest, user_request::UserRequest},
    models::user::User,
    repositories::user_repository::{add, get_all, login},
};
use crate::entities::user_response::UserResponse;
use rocket::http::{Cookie, CookieJar};
use crate::entities::user_error::UserError;


#[derive(Responder)]
#[response(content_type = "json")]
pub enum ResponseStatus<T> {
    #[response(status = 200)]
    Accepted(T),
    #[response(status = 405)]
    BadRequest(T),
    #[response(status = 404)]
    NotFound(T),
    #[response(status = 500)]
    ServerError(T),
}

#[get("/")]
pub async fn get_all_users() -> Json<Vec<User>> {
    let users: Vec<User> = get_all().await;
    Json(users)
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
        Err(e) => handle_error(e)
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
        Err(e) => handle_error(e)
    }
}

fn handle_error(e: UserError) -> ResponseStatus<Json<UserResponse>> {
    match e {
        UserError::FatalQueryError => ResponseStatus::ServerError(Json(UserResponse {
            message: e.message()
        })),
        UserError::UserNotFound => ResponseStatus::NotFound(Json(UserResponse {
            message: e.message()
        })),
        _ => ResponseStatus::BadRequest(Json(UserResponse {
            message: e.message()
        })),
    }
}

fn set_login_cookie(username: &String, jar: &CookieJar<'_>) {
    let cookie = Cookie::build("login", username.clone())
        .domain("http://localhost:8000".to_string())
        .path("/")
        .http_only(true)
        .finish();
    jar.add(cookie);
}
