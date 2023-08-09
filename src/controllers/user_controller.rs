use rocket::http::CookieJar;
use rocket::serde::json::Json;

use crate::{
    models::user::User,
    repositories::user_repository::{add, get_all, login},
};
use crate::controllers::{check_login_cookie, ResponseStatus, set_login_cookie};
use crate::entities::authorization::api_auth_error::{ApiAuthResponse, ApiAuthStatus, ApiKey};
use crate::entities::Construct;
use crate::entities::user::login_request::LoginRequest;
use crate::entities::user::user_error::UserError;
use crate::entities::user::user_request::UserRequest;
use crate::entities::user::user_response::{UserResponse, UserResponseWith};
use crate::repositories::user_repository::login_with_cookie;

#[get("/")]
pub async fn get_all_users(
    key: Result<
        ApiKey<'_>,
        ApiAuthStatus
    >
) -> Result<
    ResponseStatus<Json<UserResponseWith<Vec<User>>>>,
    ResponseStatus<Json<ApiAuthResponse>>
> {
    match key {
        Ok(_) => {
            let users: Box<Vec<User>> = Box::new(get_all().await);
            Ok(ResponseStatus::AcceptedContent(
                Json(
                    UserResponseWith {
                        message: "succ".to_string(),
                        content: users,
                    }
                )
            ))
        }
        Err(e) => Err(e.construct())
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

#[get("/login", format = "json")]
pub async fn login_user_with_cookie(jar: &CookieJar<'_>) -> ResponseStatus<Json<UserResponse>> {
    match check_login_cookie(jar) {
        Ok(username) => {
            match login_with_cookie(username).await {
                Ok(username) => {
                    set_login_cookie(&username, jar);
                    ResponseStatus::Accepted(Json(UserResponse {
                        message: format!("User {} successfully logged in (using a login cookie!!)!", username)
                    }))
                }
                Err(e) => e.construct()
            }
        }
        Err(_) => UserError::FatalQueryError.construct()
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
