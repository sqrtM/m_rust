use rocket::http::{Cookie, CookieJar};

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

fn set_login_cookie(username: &String, jar: &CookieJar<'_>) {
    let cookie = Cookie::build("login", username.clone())
        .domain("http://localhost:8000".to_string())
        .path("/")
        .http_only(true)
        .finish();
    jar.add_private(cookie);
}

fn check_login_cookie(jar: &CookieJar) -> Result<String, ()> {
    match jar.get_private("login") {
        None => Err(()),
        Some(cookie) => Ok(cookie.value().to_string())
    }
}
