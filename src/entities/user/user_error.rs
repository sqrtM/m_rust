use rocket::serde::json::Json;
use crate::controllers::ResponseStatus;
use crate::entities::Construct;
use crate::entities::user::user_response::UserResponse;

pub enum UserError {
    UsernameTaken,
    EmailTaken,
    UserNotFound,
    DuplicateEmail,
    FatalQueryError,
    AuthenticationError,
}

impl Construct<UserResponse> for UserError {
    fn message(&self) -> String {
        match self {
            UserError::UsernameTaken => "This username is already taken!",
            UserError::EmailTaken => "This email is already taken!",
            UserError::UserNotFound => "Email or password is incorrect!",
            UserError::DuplicateEmail => "Oh no, multiple users with this email exist. Panicking...",
            UserError::FatalQueryError => "Internal error finding what you were looking for... Sorry!",
            UserError::AuthenticationError => "Authentication failure!"
        }.to_string()
    }

    fn construct(&self) -> ResponseStatus<Json<UserResponse>> {
        match self {
            UserError::FatalQueryError => ResponseStatus::ServerError(Json(UserResponse {
                message: self.message()
            })),
            UserError::UserNotFound => ResponseStatus::NotFound(Json(UserResponse {
                message: self.message()
            })),
            UserError::AuthenticationError => ResponseStatus::Unauthorized(Json(UserResponse {
                message: self.message()
            })),
            _ => ResponseStatus::BadRequest(Json(UserResponse {
                message: self.message()
            })),
        }
    }
}