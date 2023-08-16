use rocket::serde::json::Json;
use crate::controllers::ResponseStatus;
use crate::entities::authorization::api_auth_error::{ApiAuthResponse, ApiAuthStatus, ApiKey};
use crate::entities::character::character_response::CharacterResponseWith;
use crate::entities::Construct;
use crate::models::character::Character;
use crate::repositories::character_repository::get_all;

#[get("/")]
pub async fn get_all_characters(
    key: Result<ApiKey<'_>, ApiAuthStatus>,
) -> Result<ResponseStatus<Json<CharacterResponseWith<Vec<Character>>>>, ResponseStatus<Json<ApiAuthResponse>>>
{
    match key {
        Ok(_) => {
            let characters: Box<Vec<Character>> = Box::new(get_all().await);
            Ok(ResponseStatus::Accepted(Json(CharacterResponseWith {
                message: "success".to_string(),
                content: characters,
            })))
        }
        Err(e) => Err(e.construct()),
    }
}