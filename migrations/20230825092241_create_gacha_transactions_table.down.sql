ALTER TABLE gacha_transactions
    DROP CONSTRAINT fk_gacha_transaction_to_user;

ALTER TABLE gacha_transactions
    DROP CONSTRAINT fk_gacha_transaction_to_gacha_reward;

DROP TABLE IF EXISTS gacha_transactions;
