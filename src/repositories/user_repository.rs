use sqlx::{Pool, Postgres};

use crate::{
    db,
    entities::{login_request::LoginRequest, user_request::UserRequest},
    models::user::User,
};

pub async fn get_all() -> Vec<User> {
    let pool: Pool<Postgres> = db().await;
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("error with query")
}

pub async fn add(request: &UserRequest) -> Result<u64, String> {
    let pool: Pool<Postgres> = db().await;
    match check_duplicate_emails(&request.email, &pool).await {
        Ok(_) => Ok(insert_new_user(&request, &pool).await?),
        Err(_) => Err("Given Email is already in use!".to_string()),
    }
}

pub async fn login(request: LoginRequest) -> User {
    let pool: Pool<Postgres> = db().await;
    sqlx::query_as!(
        User,
        "
        UPDATE users 
        SET last_login = $1
        WHERE email = crypt($2, email)
        AND password = crypt($3, password)
        RETURNING *;
        ",
        chrono::offset::Utc::now(),
        request.email,
        request.password,
    )
        .fetch_one(&pool)
        .await
        .expect("error with query")
}

struct UserId {
    user_id: i32,
}

async fn check_duplicate_emails(email: &str, pool: &Pool<Postgres>) -> Result<(), String> {
    match sqlx::query_as!(
        UserId,
        "
        SELECT user_id
        FROM users
        WHERE email = crypt($1, email);
        ",
        email
    ).fetch_one(pool).await {
        Ok(_) => Err("Email already taken!".to_string()),
        Err(_) => Ok(())
    }
}

async fn insert_new_user(request: &UserRequest, pool: &Pool<Postgres>) -> Result<u64, String> {
    match sqlx::query_as!(
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
            crypt($3, gen_salt('bf')),
            $4,
            $5
        );
        ",
        request.username,
        request.password,
        request.email,
        chrono::offset::Utc::now(),
        chrono::offset::Utc::now(),
    )
        .execute(pool)
        .await {
        Ok(res) => Ok(res.rows_affected()),
        Err(e) => Err("Username already Taken".to_string()),
    }
}
