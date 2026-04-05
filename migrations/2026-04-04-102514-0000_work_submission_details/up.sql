
CREATE TABLE submission_details (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    worker_wallet_address varchar NOT NULL REFERENCES users(user_wallet_address),
    task_id UUID NOT NULL REFERENCES tasks(id),
    selected_option_id UUID NOT NULL REFERENCES task_options(id)
);

-- index on user wallet address for fast search by user wallet address
CREATE INDEX idx_submission_wallet_id on submission_details(worker_wallet_address);
CREATE INDEX idx_submission_task_id on submission_details(task_id);



