-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_submission_wallet_id;
DROP INDEX IF EXISTS idx_submission_task_id;
DROP TABLE IF EXISTS submission_details;
