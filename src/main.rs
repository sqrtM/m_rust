#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};
use sqlx::{Pool, Postgres};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .mount("/", routes![hello])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result<Rocket<Build>> {
    println!("Attempting to migrate the database...");
    let pool: Pool<Postgres> = Pool::connect("postgresql://postgres:password@m_rust-database-1/db")
        .await
        .expect("Problem connecting");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("problem migrating");
    println!("Migration successful! Launching...");
    Ok(rocket)
}
