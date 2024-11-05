-- Add migration script here

CREATE TABLE IF NOT EXISTS blocks
(
    id          TEXT PRIMARY KEY NOT NULL,
    kind        TEXT NOT NULL, -- block type, such as: page, text, to-do, list, etc
    parent_id   TEXT, -- parent ID that this block belongs to
    children    TEXT, -- child IDs ordered. Missing IDs sort last.
    props       TEXT,
    start       TEXT, -- start datetime
    end         TEXT, -- end datetime
    done        BOOLEAN NOT NULL DEFAULT 0, -- completed task
    FOREIGN KEY(parent_id) REFERENCES blocks(id)
);


