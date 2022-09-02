-- Your SQL goes here
CREATE TABLE trip_locations (
    id          INTEGER     NOT NULL PRIMARY KEY,
    location_of INTEGER     NOT NULL,
    name        TEXT        NOT NULL,
    weather     INTEGER,
    updated_at  INTEGER     NOT NULL,
    created_at  INTEGER     NOT NULL
)