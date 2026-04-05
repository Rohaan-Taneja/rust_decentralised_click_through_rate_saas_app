-- Your SQL goes here


CREATE TABLE task_options (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(id),
    image_url VARCHAR NOT NULL ,
    no_of_times_image_selected smallint NOT NULL DEFAULT 0
);

create INDEX idx_task_id ON task_options(task_id);