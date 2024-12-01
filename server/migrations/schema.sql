CREATE TABLE IF NOT EXISTS users
(
    user_id  BIGSERIAL PRIMARY KEY,
    username TEXT      UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts
(
    account_id    BIGSERIAL PRIMARY KEY,
    user_id       BIGINT,
    account_name  TEXT,
    account_type  TEXT,
    account_limit DOUBLE PRECISION,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS transactions
(
    transaction_id   BIGSERIAL PRIMARY KEY,
    transaction_date DATE,
    transaction_type TEXT,
    category         TEXT,
    amount           DOUBLE PRECISION,
    transaction_memo TEXT,
    account_id       BIGINT,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id) ON DELETE CASCADE
);
