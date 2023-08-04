use sqlx::{Pool, Postgres};

use crate::{db, entities::user_request::UserRequest, models::user::User};

pub async fn get_all() -> Vec<User> {
    let pool: Pool<Postgres> = db().await;
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("error with query")
}

pub async fn add(request: UserRequest) -> User {
    let pool: Pool<Postgres> = db().await;
    sqlx::query_as!(
        User,
        "
        INSERT INTO users (
            username,
            password,
            email,
            created_on,
            last_login
        ) VALUES (
            $1,
            crypt($2, gen_salt('bf')),
            $3,
            $4,
            $5
        ) RETURNING *;
        ",
        request.username,
        request.password,
        request.email,
        chrono::offset::Utc::now(),
        chrono::offset::Utc::now(),
    )
    .fetch_one(&pool)
    .await
    .expect("error with query")
}
