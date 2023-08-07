use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}
