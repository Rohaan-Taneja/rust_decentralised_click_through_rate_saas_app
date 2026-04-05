-- Your SQL goes here


CREATE TYPE user_type AS ENUM ('CREATOR' , 'WORKER');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_wallet_addres VARCHAR NOT NULL , 
    type user_type NOT NULL
);