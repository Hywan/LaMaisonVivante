use crate::unit::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum VentilationState {
    Auto,
    Cooling,
    Heating,
}

#[derive(Debug, Serialize)]
pub struct AirThroughput {
    pub supplied_air_fan_speed: Rpm,
    pub extracted_air_fan_speed: Rpm,
}

#[derive(Debug, Serialize)]
pub struct AirTemperatures {
    pub supplied_air_after_ground_coupled_heat_exchanger: Degree,
    pub supplied_air_after_heat_recovery_exchanger: Degree,
    pub extracted_air: Degree,
    pub discharged_air: Degree,
    pub wanted_inside_air: Degree,
}

#[derive(Debug, Serialize)]
pub struct Ventilation {
    pub state: VentilationState,
    pub air_throughput: AirThroughput,
    pub inside_air_humidity: Percent,
    pub inside_co2_level: Ppm,
    pub temperatures: AirTemperatures,
}

#[derive(Debug, Serialize)]
pub enum AntiLegionellaFrequency {
    Off,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize)]
pub struct AntiLegionella {
    pub frequency: AntiLegionellaFrequency,
    pub day: u16,
    pub hour: u16,
}

#[derive(Debug, Serialize)]
pub struct StorageHotWaterTemperatures {
    pub top_of_the_tank: Degree,
    pub bottom_of_the_tank: Degree,
    pub wanted: Degree,
}

#[derive(Debug, Serialize)]
pub struct DomesticHotWater {
    pub anti_legionella: AntiLegionella,
    pub storage_temperatures: StorageHotWaterTemperatures,
}

#[derive(Debug, Serialize)]
pub struct State {
    pub ventilation: Ventilation,
    pub domestic_hot_water: DomesticHotWater,
}
