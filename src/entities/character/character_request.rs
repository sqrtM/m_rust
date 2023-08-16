use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CharacterRequest {
    pub character_name: String,
    pub constitution: i32,
    pub strength: i32,
    pub madness: i32,
    pub intelligence: i32,
}
