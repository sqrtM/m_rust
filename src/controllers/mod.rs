use std::time::SystemTime;

use chrono::{DateTime, Utc};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::time::OffsetDateTime;
use uuid::Uuid;

use crate::repositories::cookie_repository::register_new_login_cookie;

pub mod user_controller;

#[derive(Responder)]
#[response(content_type = "json")]
pub enum ResponseStatus<T> {
    #[response(status = 200)]
    Accepted(T),
    #[response(status = 401)]
    Unauthorized(T),
    #[response(status = 404)]
    NotFound(T),
    #[response(status = 405)]
    BadRequest(T),
    #[response(status = 500)]
    ServerError(T),
}

async fn set_login_cookie(user_id: i32, jar: &CookieJar<'_>) {
    let uuid = Uuid::new_v4().as_hyphenated().to_string();

    let expires: DateTime<Utc> = register_new_login_cookie(&uuid, user_id)
        .await
        .expect("Error registering login cookie.");

    let cookie = Cookie::build("login", uuid)
        .path("/users")
        .secure(true)
        .same_site(SameSite::None)
        .expires(
            OffsetDateTime::from(
                SystemTime::from(
                    expires
                )
            )
        )
        .finish();

    jar.add_private(cookie);
}

fn check_login_cookie(jar: &CookieJar) -> Result<String, ()> {
    match jar.get_private("login") {
        None => Err(()),
        Some(cookie) => {
            Ok(cookie.value().to_string())
        }
    }
}
