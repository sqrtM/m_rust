mod controllers;
mod models;
mod repositories;

#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Build, Request, Response, Rocket};
use sqlx::{Pool, Postgres};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .attach(CORS)
        .mount(
            "/users",
            routes![controllers::user_controller::get_all_users],
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

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub async fn db() -> Pool<Postgres> {
    Pool::connect("postgresql://postgres:password@m_rust-database-1/db")
        .await
        .expect("Problem connecting")
}
