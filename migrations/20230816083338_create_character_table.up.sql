CREATE TABLE IF NOT EXISTS characters
(
    character_id   serial PRIMARY KEY       NOT NULL,
    user_id        INT UNIQUE               NOT NULL,
    CONSTRAINT fk_character_to_user
        FOREIGN KEY (user_id)
            REFERENCES users (user_id),
    character_name VARCHAR(70)              NOT NULL,
    constitution   INTEGER                  NOT NULL,
    strength       INTEGER                  NOT NULL,
    madness        INTEGER                  NOT NULL,
    intelligence   INTEGER                  NOT NULL,
    created_at     TIMESTAMP WITH TIME ZONE NOT NULL
);
