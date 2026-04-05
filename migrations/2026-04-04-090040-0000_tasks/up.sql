-- Your SQL goes here
CREATE TYPE task_status AS ENUM ('ACTIVE', 'COMPLETED');
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_wallet_address varchar NOT NULL REFERENCES users(user_wallet_address),
    txn_hash VARCHAR NOT NULL,
    is_active task_status NOT NULL,
    no_of_times_taks_done smallint NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- index on user wallet address for fast search by user wallet address
CREATE INDEX idx_users_email on tasks(user_wallet_address);


