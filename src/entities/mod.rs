use crate::controllers::ResponseStatus;
use rocket::serde::json::Json;

pub mod authorization;
pub mod user;
pub mod character;

pub trait Construct<T> {
    fn message(&self) -> String;
    fn construct(&self) -> ResponseStatus<Json<T>>;
}
