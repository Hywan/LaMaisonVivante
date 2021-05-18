-- Set of data about Domestic Hot Water.
CREATE TABLE IF NOT EXISTS domestic_hot_water (
    time TIMESTAMP WITHOUT TIME ZONE NOT NULL PRIMARY KEY,

    top_of_the_tank_temperature DOUBLE PRECISION NOT NULL,
    bottom_of_the_tank_temperature DOUBLE PRECISION NOT NULL,
    wanted_temperature DOUBLE PRECISION NOT NULL
);

-- Turn `domestic_hot_water` into a hypertable.
SELECT create_hypertable('domestic_hot_water', 'time');
