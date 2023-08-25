CREATE TABLE IF NOT EXISTS gacha_transactions
(
    gacha_transaction_id serial PRIMARY KEY       NOT NULL,
    user_id              INT UNIQUE               NOT NULL,
    CONSTRAINT fk_gacha_transaction_to_user
        FOREIGN KEY (user_id)
            REFERENCES users (user_id),
    gacha_reward_id      INT UNIQUE               NOT NULL,
    CONSTRAINT fk_gacha_transaction_to_gacha_reward
        FOREIGN KEY (gacha_reward_id)
            REFERENCES gacha_rewards (gacha_reward_id),
    created_at           TIMESTAMP WITH TIME ZONE NOT NULL
);
