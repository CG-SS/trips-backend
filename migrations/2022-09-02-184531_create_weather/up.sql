-- Your SQL goes here
CREATE TABLE weather (
    id          INTEGER     NOT NULL PRIMARY KEY,
    weather_for INTEGER     NOT NULL,
    avg_temp    FLOAT       NOT NULL,
    rain_chance FLOAT       NOT NULL,
    created_at  INTEGER     NOT NULL
)