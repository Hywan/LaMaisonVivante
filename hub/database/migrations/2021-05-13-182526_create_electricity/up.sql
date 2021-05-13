-- Electricity is produced by PV panels. The PV inverter provides
-- 3 phases. This table contains the voltage (V), frequency (Hz),
-- power (W), and current (A) for each phase (prefixed resp. as `l1_`,
-- `l2_` and `l3_`), and the sum of all phases.
CREATE TABLE IF NOT EXISTS electricity_production (
   time TIMESTAMP WITHOUT TIME ZONE NOT NULL PRIMARY KEY,

   -- PV inverter, phase 1.
   l1_voltage DOUBLE PRECISION NOT NULL,
   l1_frequency DOUBLE PRECISION NOT NULL,
   l1_power DOUBLE PRECISION NOT NULL,
   l1_current DOUBLE PRECISION NOT NULL,

   -- PV inverter, phase 2.
   l2_voltage DOUBLE PRECISION NOT NULL,
   l2_frequency DOUBLE PRECISION NOT NULL,
   l2_power DOUBLE PRECISION NOT NULL,
   l2_current DOUBLE PRECISION NOT NULL,

   -- PV inverter, phase 3.
   l3_voltage DOUBLE PRECISION NOT NULL,
   l3_frequency DOUBLE PRECISION NOT NULL,
   l3_power DOUBLE PRECISION NOT NULL,
   l3_current DOUBLE PRECISION NOT NULL,

   -- PV inverter, all phases.
   voltage DOUBLE PRECISION NOT NULL,
   frequency DOUBLE PRECISION NOT NULL,
   power DOUBLE PRECISION NOT NULL,
   current DOUBLE PRECISION NOT NULL
);

-- Turn `electricity_production` into a hypertable.
SELECT create_hypertable('electricity_production', 'time');


-- Electricity is stored in a battery. This table contains the ongoing
-- power (W), the temperature (°C), the state of charge (%) and the
-- voltage (V) of the battery.
CREATE TABLE IF NOT EXISTS electricity_storage (
   time TIMESTAMP WITHOUT TIME ZONE NOT NULL PRIMARY KEY,

   ongoing_power DOUBLE PRECISION NOT NULL,
   temperature DOUBLE PRECISION NOT NULL,
   state_of_charge DOUBLE PRECISION NOT NULL,
   voltage DOUBLE PRECISION NOT NULL
);

-- Turn `electricity_storage` into a hypertable.
SELECT create_hypertable('electricity_storage', 'time');


-- Electricity is consumed by the house. This table contains the power
-- (W) used by the house for all the 3 phases, plus the sum of all
-- phases.
CREATE TABLE IF NOT EXISTS electricity_consumption (
   time TIMESTAMP WITHOUT TIME ZONE NOT NULL PRIMARY KEY,

   house_power DOUBLE PRECISION NOT NULL,
   house_l1_power DOUBLE PRECISION NOT NULL,
   house_l2_power DOUBLE PRECISION NOT NULL,
   house_l3_power DOUBLE PRECISION NOT NULL
);

-- Turn `electricity_consumption` into a hypertable.
SELECT create_hypertable('electricity_consumption', 'time');
