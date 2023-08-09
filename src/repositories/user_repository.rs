use sqlx::{Pool, Postgres};

use crate::{
    db,
    models::user::User,
};
use crate::entities::user::login_request::LoginRequest;
use crate::entities::user::user_error::UserError;
use crate::entities::user::user_request::UserRequest;

struct Username {
    username: String,
}

pub async fn get_all() -> Vec<User> {
    let pool: Pool<Postgres> = db().await;
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("error with query")
}

pub async fn add(request: &UserRequest) -> Result<String, UserError> {
    let pool: Pool<Postgres> = db().await;
    check_duplicate_emails(&request.email, &pool).await?;
    insert_new_user(&request, &pool).await
}

pub async fn login(request: LoginRequest) -> Result<String, UserError> {
    let pool: Pool<Postgres> = db().await;
    match sqlx::query_as!(
        Username,
        "
        UPDATE users 
        SET last_login = $1
        WHERE email = crypt($2, email)
        AND password = crypt($3, password)
        RETURNING username;
        ",
        chrono::offset::Utc::now(),
        request.email,
        request.password,
    )
        .fetch_all(&pool)
        .await {
        Ok(res) => match res.len() {
            0 => Err(UserError::UserNotFound),
            1 => match res.get(0) {
                None => Err(UserError::UserNotFound),
                Some(user) => Ok(user.username.clone())
            },
            _ => Err(UserError::DuplicateEmail)
        },
        Err(_) => Err(UserError::FatalQueryError)
    }
}

pub async fn login_with_cookie(username: String) -> Result<String, UserError> {
    let pool: Pool<Postgres> = db().await;
    match sqlx::query_as!(
        Username,
        "
        UPDATE users
        SET last_login = $1
        WHERE username = $2
        RETURNING username;
        ",
        chrono::offset::Utc::now(),
        username
    )
        .fetch_all(&pool)
        .await {
        Ok(res) => match res.len() {
            0 => Err(UserError::UserNotFound),
            1 => match res.get(0) {
                None => Err(UserError::UserNotFound),
                Some(user) => Ok(user.username.clone())
            },
            _ => Err(UserError::DuplicateEmail)
        },
        Err(_) => Err(UserError::FatalQueryError)
    }
}

async fn check_duplicate_emails(email: &str, pool: &Pool<Postgres>) -> Result<(), UserError> {
    match sqlx::query_as!(
        Username,
        "
        SELECT username
        FROM users
        WHERE email = crypt($1, email);
        ",
        email
    ).fetch_one(pool).await {
        Ok(_) => Err(UserError::EmailTaken),
        Err(_) => Ok(())
    }
}

async fn insert_new_user(request: &UserRequest, pool: &Pool<Postgres>) -> Result<String, UserError> {
    match sqlx::query_as!(
        Username,
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
        )
        RETURNING username;
        ",
        request.username,
        request.password,
        request.email,
        chrono::offset::Utc::now(),
        chrono::offset::Utc::now(),
    )
        .fetch_one(pool)
        .await {
        Ok(user) => Ok(user.username.clone()),
        Err(_) => Err(UserError::UsernameTaken),
    }
}
