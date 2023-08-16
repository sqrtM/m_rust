#[macro_use]
extern crate rocket;

use rocket::{Build, Request, Response, Rocket};
use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use sqlx::{Pool, Postgres};

mod controllers;
mod entities;
mod models;
mod repositories;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .attach(CORS)
        .mount(
            "/users",
            routes![
                controllers::user_controller::get_all_users,
                controllers::user_controller::add_user,
                controllers::user_controller::login_user,
                controllers::user_controller::login_user_with_cookie
            ],
        )
        .mount(
            "/characters",
            routes![
                controllers::character_controller::get_all_characters
            ]
        )
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result<Rocket<Build>> {
    println!("Attempting to migrate the database...");
    let pool: Pool<Postgres> = db().await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("problem migrating");
    println!("Migration successful! Launching...");
    Ok(rocket)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "content-type, authorization",
            ));
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Vary", "Origin"));
    }
}

pub async fn db() -> Pool<Postgres> {
    Pool::connect("postgresql://postgres:password@m_rust-database-1/db")
        .await
        .expect("Problem connecting")
}
