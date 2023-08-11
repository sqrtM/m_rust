use crate::controllers::ResponseStatus;
use crate::entities::Construct;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::Request;

#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[derive(Debug, Serialize)]
pub enum ApiAuthStatus {
    Missing,
    Invalid,
}

#[derive(Serialize, Responder, Debug)]
#[response(status = 401, content_type = "json")]
pub struct ApiAuthResponse {
    message: String,
}

impl Construct<ApiAuthResponse> for ApiAuthStatus {
    fn message(&self) -> String {
        match self {
            ApiAuthStatus::Missing => "No bearer token!",
            ApiAuthStatus::Invalid => "Invalid bearer token!",
        }
        .to_string()
    }

    fn construct(&self) -> ResponseStatus<Json<ApiAuthResponse>> {
        match self {
            ApiAuthStatus::Missing => ResponseStatus::Unauthorized(Json(ApiAuthResponse {
                message: self.message(),
            })),
            ApiAuthStatus::Invalid => ResponseStatus::Unauthorized(Json(ApiAuthResponse {
                message: self.message(),
            })),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiAuthStatus;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ApiAuthStatus> {
        fn is_valid(key: &str) -> bool {
            key == "Bearer token"
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, ApiAuthStatus::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiAuthStatus::Invalid)),
        }
    }
}
