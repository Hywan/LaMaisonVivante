-- Set of data around air quality.
CREATE TABLE IF NOT EXISTS air (
    time TIMESTAMP WITHOUT TIME ZONE NOT NULL PRIMARY KEY,

    -- Humidity.
    inside_humidity DOUBLE PRECISION NOT NULL,

    -- Temperatures.
    supplied_temperature_after_ground_coupled_heat_exchanger DOUBLE PRECISION NOT NULL,
    supplied_temperature_after_heat_recovery_exchanger DOUBLE PRECISION NOT NULL,
    extracted_temperature DOUBLE PRECISION NOT NULL,
    discharged_temperature DOUBLE PRECISION NOT NULL,
    wanted_temperature DOUBLE PRECISION NOT NULL
);

-- Turn `air` into a hypertable.
SELECT create_hypertable('air', 'time');
