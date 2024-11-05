-- Add migration script here

CREATE TABLE IF NOT EXISTS blocks
(
    id          TEXT PRIMARY KEY NOT NULL,
    type        TEXT NOT NULL, -- block type, such as: page, text, to-do, list, etc
    parent_id   TEXT, -- parent ID that this block belongs to
    content     TEXT, -- child IDs ordered. Missing IDs sort last.
    props       BLOB,
    start       TEXT,
    end         TEXT,
    done        BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY(parent_id) REFERENCES blocks(id)
);


