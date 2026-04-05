-- Your SQL goes here

CREATE TYPE pay_status AS ENUM ('LOCKED' , 'NOT_ELIGIBLE' , 'CAN_WITHDRAW');

CREATE TABLE worker_work_details (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    worker_wallet_address VARCHAR NOT NULL REFERENCES users(user_wallet_address),
    total_lifetime_tasks  INTEGER NOT NULL DEFAULT 0 ,
    current_no_of_tasks_for_payout INTEGER NOT NULL DEFAULT 0 ,
    payout_status pay_status NOT NULL ,
    txn_hash_of_withdrawal VARCHAR 


);

-- index on user wallet address for fast search by user wallet address
CREATE INDEX idx_worker_Wallet on worker_work_details(worker_wallet_address);
