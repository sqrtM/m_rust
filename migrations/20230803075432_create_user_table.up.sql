CREATE TABLE IF NOT EXISTS users
(
    user_id    SERIAL PRIMARY KEY       NOT NULL,
    username   VARCHAR(50) UNIQUE       NOT NULL,
    password   VARCHAR(60)              NOT NULL,
    email      VARCHAR(60) UNIQUE       NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE NOT NULL,
    last_login TIMESTAMP WITH TIME ZONE NOT NULL
);
