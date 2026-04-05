--  rename column from user_wallet_addres to user_wallet_address
ALTER TABLE users RENAME COLUMN user_wallet_addres TO user_wallet_address;

-- Add UNIQUE constraint
ALTER TABLE users
ADD CONSTRAINT users_user_wallet_address_unique UNIQUE (user_wallet_address);

-- index on wallet address field
CREATE INDEX idx_users_wallet_address
ON users(user_wallet_address);