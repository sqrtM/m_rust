use serde::Serialize;

#[derive(Serialize, Responder, Debug)]
pub struct UserResponseWith<T> {
    pub message: String,
    pub content: Box<T>,
}

#[derive(Serialize, Responder, Debug)]
pub struct UserResponse {
    pub message: String,
}
