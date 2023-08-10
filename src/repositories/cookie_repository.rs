use chrono::{DateTime, Duration, Utc};
use sqlx::{Pool, Postgres};
use crate::db;
use crate::entities::user::user_error::UserError;
use crate::models::user::User;

pub async fn register_new_login_cookie(cookie: &str, user_id: i32) -> Result<DateTime<Utc>, UserError> {
    let pool: Pool<Postgres> = db().await;
    struct ExpiresAt {
        expires_at: DateTime<Utc>,
    }

    match sqlx::query_as!(
        ExpiresAt,
        "
        INSERT INTO
            login_cookies (
                        cookie,
                        user_id,
                        expires_at,
                        created_on
            ) VALUES (
                        crypt($1, gen_salt('bf')),
                        $2,
                        $3,
                        $4
            ) RETURNING expires_at
        ",
        cookie,
        user_id,
        chrono::offset::Utc::now() + Duration::days(1),
        chrono::offset::Utc::now(),
    )
        .fetch_one(&pool).await {
        Ok(res) => Ok(res.expires_at),
        Err(_) => Err(UserError::FatalQueryError)
    }
}

pub async fn login_with_cookie(cookie: &str) -> Result<User, UserError> {
    let pool: Pool<Postgres> = db().await;

    match sqlx::query_as!(
        User,
        "
        SELECT
            users.user_id,
            users.username,
            users.password,
            users.email,
            users.created_on,
            users.last_login
        FROM
            users
        LEFT JOIN
            login_cookies ON users.user_id = login_cookies.user_id
        WHERE
            login_cookies.cookie = crypt($1, cookie)
        ",
        cookie
    ).fetch_one(&pool).await {
        Ok(user) => Ok(user),
        Err(_) => Err(UserError::FatalQueryError)
    }
}