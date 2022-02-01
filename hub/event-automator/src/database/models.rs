use super::{enums::*, schema::*};
use diesel::*;
use std::time::SystemTime;

#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "electricity_production"]
pub struct ElectricityProduction {
    pub voltage: f64,
    pub frequency: f64,
    pub power: f64,
    pub current: f64,
}

#[derive(Queryable, Debug)]
pub struct ElectricityStorage {
    pub ongoing_power: f64,
    pub temperature: f64,
    pub state_of_charge: f64,
    pub voltage: f64,
}

#[derive(Queryable, Debug)]
pub struct ElectricityConsumption {
    pub house_power: f64,
}

#[derive(Queryable, Debug)]
pub struct DomesticHotWater {
    pub top_of_the_tank_temperature: f64,
    pub bottom_of_the_tank_temperature: f64,
}

#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "air"]
pub struct Air {
    pub time: SystemTime,
    pub inside_humidity: f64,
    pub supplied_temperature_after_ground_coupled_heat_exchanger: f64,
    pub supplied_temperature_after_heat_recovery_exchanger: f64,
    pub extracted_temperature: f64,
    pub discharged_temperature: f64,
    pub wanted_temperature: f64,
    pub state: Option<AirState>,
}
