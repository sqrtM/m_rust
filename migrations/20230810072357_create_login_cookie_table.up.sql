CREATE TABLE IF NOT EXISTS login_cookies
(
    cookie_id  serial PRIMARY KEY       NOT NULL,
    cookie     VARCHAR(60)              NOT NULL,
    user_id    INT UNIQUE               NOT NULL,
    CONSTRAINT fk_cookie_to_user
        FOREIGN KEY (user_id)
            REFERENCES users (user_id),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL
);
