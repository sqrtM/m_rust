use serde::Serialize;

#[derive(Serialize, Responder, Debug)]
pub struct UserResponse {
    pub message: String,
}
