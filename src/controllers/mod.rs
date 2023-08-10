use std::time::SystemTime;

use chrono::{DateTime, Utc};
use rocket::config::SecretKey;
use rocket::http::{Cookie, CookieJar};
use rocket::time::OffsetDateTime;

use crate::repositories::cookie_repository::register_new_login_cookie;

pub mod user_controller;

#[derive(Responder)]
#[response(content_type = "json")]
pub enum ResponseStatus<T> {
    #[response(status = 200)]
    AcceptedContent(T),
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
    let secret_key = match SecretKey::generate() {
        None => panic!("no secret key"),
        Some(key) => key.to_string(),
    };

    let expires: DateTime<Utc> = register_new_login_cookie(&secret_key, user_id)
        .await
        .expect("TODO: panic message");

    let cookie = Cookie::build("login", secret_key)
        .domain("http://localhost:8000".to_string())
        .path("/")
        .http_only(true)
        .expires(OffsetDateTime::from(SystemTime::from(expires)))
        .finish();
    jar.add_private(cookie);
}

fn check_login_cookie(jar: &CookieJar) -> Result<String, ()> {
    match jar.get_private("login") {
        None => Err(()),
        Some(cookie) => Ok(cookie.value().to_string())
    }
}
