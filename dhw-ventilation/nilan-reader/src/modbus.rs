/// Ventilation state.
pub const VENTILATION_STATE: u16 = 21770;

/// Fan speed for supplied air.
pub const SUPPLIED_AIR_FAN_SPEED: u16 = 21771;

/// Fan speed for extracted air.
pub const EXTRACTED_AIR_FAN_SPEED: u16 = 21772;

/// Inside air humidity.
pub const INSIDE_AIR_HUMIDITY: u16 = 20164;

/// Inside CO_2 level.
pub const INSIDE_CO2_LEVEL: u16 = 21778;

/// Air temperature outside the house.
// pub const missing

/// Air goes through a ground-coupled heat exchanger, before being
/// injected inside the house…
pub const SUPPLIED_AIR_TEMPERATURE_AFTER_GROUND_COUPLED_HEAT_EXCHANGER: u16 = 20282; // T1

/// …. It then goes through the heat recovery
/// exchanger, to finally be supplied inside the house.
pub const SUPPLIED_AIR_TEMPERATURE_AFTER_HEAT_RECOVERY_EXCHANGER: u16 = 20284; // T2

/// Extracted air from inside the house.
pub const EXTRACTED_AIR_TEMPERATURE: u16 = 20286; // T3

/// Extracted air after the heat recovery exchanger.
pub const DISCHARGED_AIR_TEMPERATURE: u16 = 20288; // T4

/// Targeted inside air temperature.
pub const WANTED_INSIDE_AIR_TEMPERATURE: u16 = 20260;

/// Automatic anti-legionella.
pub const AUTOMATIC_ANTI_LEGIONELLA: u16 = 20481;

/// Day for anti-legionella.
pub const DAY_FOR_ANTI_LEGIONELLA: u16 = 20481;

/// Time for anti-legionella.
pub const TIME_FOR_ANTI_LEGIONELLA: u16 = 20483;

/// Top temperature in the domestic hot water tank.
pub const TOP_TEMPERATURE_IN_DOMESTIC_HOT_WATER_TANK: u16 = 20520;

/// Bottom temperature in the domestic hot water tank.
pub const BOTTOM_TEMPERATURE_IN_DOMESTIC_HOT_WATER_TANK: u16 = 20522;

/// Wanted hot water production.
pub const WANTED_HOT_WATER_TEMPERATURE: u16 = 20540;
