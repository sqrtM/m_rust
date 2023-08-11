use crate::controllers::ResponseStatus;
use crate::entities::user::user_response::UserResponse;
use crate::entities::Construct;
use rocket::serde::json::Json;

#[derive(Debug)]
pub enum UserError {
    UsernameTaken,
    EmailTaken,
    UserNotFound,
    DuplicateEmail,
    NoCookie,
    FatalQueryError,
}

impl Construct<UserResponse> for UserError {
    fn message(&self) -> String {
        match self {
            UserError::UsernameTaken => "This username is already taken!",
            UserError::EmailTaken => "This email is already taken!",
            UserError::UserNotFound => "Email or password is incorrect!",
            UserError::DuplicateEmail => {
                "Oh no, multiple users with this email exist. Panicking..."
            },
            UserError::NoCookie => "No Cookie Found!",
            UserError::FatalQueryError => {
                "Internal error finding what you were looking for... Sorry!"
            }
        }
        .to_string()
    }

    fn construct(&self) -> ResponseStatus<Json<UserResponse>> {
        match self {
            UserError::FatalQueryError => ResponseStatus::ServerError(Json(UserResponse {
                message: self.message(),
            })),
            UserError::UserNotFound => ResponseStatus::NotFound(Json(UserResponse {
                message: self.message(),
            })),
            _ => ResponseStatus::BadRequest(Json(UserResponse {
                message: self.message(),
            })),
        }
    }
}
