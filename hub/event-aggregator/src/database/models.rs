use super::schema::*;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "electricity_production"]
pub struct ElectricityProduction<'a> {
    pub time: &'a SystemTime,

    pub l1_voltage: f64,
    pub l1_frequency: f64,
    pub l1_power: f64,
    pub l1_current: f64,

    pub l2_voltage: f64,
    pub l2_frequency: f64,
    pub l2_power: f64,
    pub l2_current: f64,

    pub l3_voltage: f64,
    pub l3_frequency: f64,
    pub l3_power: f64,
    pub l3_current: f64,

    pub voltage: f64,
    pub frequency: f64,
    pub power: f64,
    pub current: f64,
}

#[derive(Insertable)]
#[table_name = "electricity_storage"]
pub struct ElectricityStorage<'a> {
    pub time: &'a SystemTime,

    pub ongoing_power: f64,
    pub temperature: f64,
    pub state_of_charge: f64,
    pub voltage: f64,
}

#[derive(Insertable)]
#[table_name = "electricity_consumption"]
pub struct ElectricityConsumption<'a> {
    pub time: &'a SystemTime,

    pub house_power: f64,
    pub house_l1_power: f64,
    pub house_l2_power: f64,
    pub house_l3_power: f64,
}

#[derive(Insertable)]
#[table_name = "domestic_hot_water"]
pub struct DomesticHotWater<'a> {
    pub time: &'a SystemTime,

    pub top_of_the_tank_temperature: f64,
    pub bottom_of_the_tank_temperature: f64,
    pub wanted_temperature: f64,
}

#[derive(Insertable)]
#[table_name = "air"]
pub struct Air<'a> {
    pub time: &'a SystemTime,

    pub state: String,

    pub inside_humidity: f64,

    pub supplied_temperature_after_ground_coupled_heat_exchanger: f64,
    pub supplied_temperature_after_heat_recovery_exchanger: f64,
    pub extracted_temperature: f64,
    pub discharged_temperature: f64,
    pub wanted_temperature: f64,
}
