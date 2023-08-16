use serde::Serialize;

#[derive(Serialize, Responder, Debug)]
pub struct CharacterResponseWith<T> {
    pub message: String,
    pub content: Box<T>,
}

#[derive(Serialize, Responder, Debug)]
pub struct CharacterResponse {
    pub message: String,
}
