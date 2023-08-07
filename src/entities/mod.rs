use rocket::serde::json::Json;
use crate::controllers::ResponseStatus;

pub mod user;
pub mod authorization;

pub trait Construct<T> {
    fn message(&self) -> String;
    fn construct(&self) -> ResponseStatus<Json<T>>;
}
