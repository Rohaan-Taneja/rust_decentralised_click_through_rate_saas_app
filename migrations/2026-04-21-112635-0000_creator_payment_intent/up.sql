-- // create paymentid table (store id , user_Wallet , 
-- txn_hash , status = finalised , created_at , exp = 5mins)


CREATE TYPE creator_txn_status AS ENUM ( 'CREATED' , 'FAILED' , 'COMPLETED');

CREATE TABLE creators_payment(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creator_wallet_address VARCHAR NOT NULL REFERENCES users(user_wallet_address),
    txh_sign VARCHAR UNIQUE ,
    payment_status creator_txn_status NOT NULL DEFAULT 'CREATED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP


);

CREATE INDEX idx_payment_id on creators_payment(id);

CREATE INDEX idx_payment_txn_sign on creators_payment(txh_sign);

CREATE INDEX idx_creator_wallet_address on creators_payment(creator_wallet_address);


