-- Drop UNIQUE constraint
ALTER TABLE users
DROP CONSTRAINT users_user_wallet_address_unique;

-- (Optional) Drop index if you explicitly created it
DROP INDEX IF EXISTS idx_users_wallet_address;

-- Rename column back
ALTER TABLE users
RENAME COLUMN user_wallet_address TO user_wallet_addres;