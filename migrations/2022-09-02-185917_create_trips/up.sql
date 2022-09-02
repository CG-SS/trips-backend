-- Your SQL goes here
CREATE TABLE trips (
    id          INTEGER     NOT NULL PRIMARY KEY,
    travelers   TEXT,
    schedule    TEXT,
    start_date  INTEGER     NOT NULL,
    finish_date INTEGER     NOT NULL,
    updated_at  INTEGER     NOT NULL,
    created_at  INTEGER     NOT NULL
)