

CREATE TYPE payout_status AS ENUM ('PENDING', 'COMPLETED' , 'FAILED');

CREATE TABLE payouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    worker_wallet_address VARCHAR NOT NULL REFERENCES users(user_wallet_address),
    amount VARCHAR NOT NULL ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP ,
    completed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP ,
    txn_hash VARCHAR NOT NULL ,
    payout_status payout_status NOT NULL

);


CREATE INDEX idx_worker_payout on tasks(txn_hash);


