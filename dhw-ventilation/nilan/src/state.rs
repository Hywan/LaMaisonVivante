use crate::unit::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum VentilationState {
    Paused,
    Running,
}

impl Default for VentilationState {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Debug, Serialize)]
pub enum VentilationMode {
    Auto,
    Cooling,
    Heating,
}

impl Default for VentilationMode {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Serialize, Default)]
pub struct AirThroughput {
    pub supplied_air_fan_speed: Rpm,
    pub extracted_air_fan_speed: Rpm,
}

#[derive(Debug, Serialize, Default)]
pub struct AirTemperatures {
    pub supplied_air_after_ground_coupled_heat_exchanger: Degree,
    pub supplied_air_after_heat_recovery_exchanger: Degree,
    pub extracted_air: Degree,
    pub discharged_air: Degree,
    pub wanted_inside_air: Degree,
}

#[derive(Debug, Serialize, Default)]
pub struct Ventilation {
    pub mode: VentilationMode,
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

impl Default for AntiLegionellaFrequency {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Debug, Serialize, Default)]
pub struct AntiLegionella {
    pub started_manually: bool,
    pub frequency: AntiLegionellaFrequency,
    pub day: u16,
    pub hour: u16,
}

#[derive(Debug, Serialize, Default)]
pub struct StorageHotWaterTemperatures {
    pub top_of_the_tank: Degree,
    pub bottom_of_the_tank: Degree,
    pub wanted: Degree,
}

#[derive(Debug, Serialize, Default)]
pub struct DomesticHotWater {
    pub anti_legionella: AntiLegionella,
    pub storage_temperatures: StorageHotWaterTemperatures,
}

#[derive(Debug, Serialize, Default)]
pub struct State {
    pub ventilation: Ventilation,
    pub domestic_hot_water: DomesticHotWater,
}
