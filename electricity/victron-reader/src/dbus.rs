// Dbus service names.

pub const DBUS_SERVICE_SYSTEM: u8 = 100;
pub const DBUS_SERVICE_BATTERY: u8 = 225;
pub const DBUS_SERVICE_PV_INVERTER: u8 = 20;
pub const DBUS_SERVICE_VEBUS: u8 = 246;

// Register addresses.

pub const SYSTEM_AC_CONSUMPTION_L1: u16 = 817;
pub const SYSTEM_AC_CONSUMPTION_L2: u16 = 818;
pub const SYSTEM_AC_CONSUMPTION_L3: u16 = 819;

pub const BATTERY_POWER: u16 = 842;
pub const BATTERY_STATE: u16 = 844;
pub const BATTERY_VOLTAGE: u16 = 259;
pub const BATTERY_TEMPERATURE: u16 = 262;
pub const BATTERY_STATE_OF_CHARGE: u16 = 266;
pub const BATTERY_STATE_OF_HEALTH: u16 = 304;

pub const PV_INVERTER_L1_VOLTAGE: u16 = 1027;
pub const PV_INVERTER_L1_CURRENT: u16 = 1028;
pub const PV_INVERTER_L1_POWER: u16 = 1029;
pub const PV_INVERTER_L2_VOLTAGE: u16 = 1031;
pub const PV_INVERTER_L2_CURRENT: u16 = 1032;
pub const PV_INVERTER_L2_POWER: u16 = 1033;
pub const PV_INVERTER_L3_VOLTAGE: u16 = 1035;
pub const PV_INVERTER_L3_CURRENT: u16 = 1036;
pub const PV_INVERTER_L3_POWER: u16 = 1037;

pub const VEBUS_OUTPUT_FREQUENCY: u16 = 21;
