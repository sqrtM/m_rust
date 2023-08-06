use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Responder, Debug)]
pub struct UserResponse {
    pub message: String,
}
