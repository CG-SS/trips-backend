-- Your SQL goes here
CREATE TABLE peoples (
    id          INTEGER     NOT NULL PRIMARY KEY,
    age         INTEGER     NOT NULL,
    name        TEXT        NOT NULL,
    traveler_at INTEGER, -- A person might be a traveler in a trip.
    updated_at  INTEGER     NOT NULL,
    created_at  INTEGER     NOT NULL
)