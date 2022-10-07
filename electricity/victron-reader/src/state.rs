use crate::unit::*;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Default, Serialize)]
pub enum BatteryState {
    #[default]
    Idle,
    Discharging,
    Charging,
}

#[derive(Debug, Default, Serialize)]
pub struct Battery {
    pub state: BatteryState,
    pub state_of_charge: Percent,
    pub ongoing_power: Watt,
    pub voltage: Volt,
    pub temperature: Degree,
    pub health: Percent,
}

#[derive(Debug, Copy, Clone, Default, Serialize)]
pub struct PvInverterPhase {
    pub voltage: Volt,
    pub current: Amp,
    pub power: Watt,
}

#[derive(Copy, Clone, Default, Serialize)]
pub struct PvInverter {
    pub l1: PvInverterPhase,
    pub l2: PvInverterPhase,
    pub l3: PvInverterPhase,
}

impl fmt::Debug for PvInverter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PvInverter")
            .field("l1", &self.l1)
            .field("l2", &self.l2)
            .field("l3", &self.l3)
            .field(
                "total_power",
                &(self.l1.power + self.l2.power + self.l3.power),
            )
            .finish()
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Vebus {
    pub frequency: Hertz,
}

#[derive(Default, Serialize)]
pub struct House {
    pub l1: Watt,
    pub l2: Watt,
    pub l3: Watt,
}

impl fmt::Debug for House {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("House")
            .field("l1", &self.l1)
            .field("l2", &self.l2)
            .field("l3", &self.l3)
            .field("total_consumption", &(self.l1 + self.l2 + self.l3))
            .finish()
    }
}

#[derive(Debug, Default, Serialize)]
pub struct State {
    pub battery: Option<Battery>,
    pub pv_inverter: Option<PvInverter>,
    pub vebus: Option<Vebus>,
    pub house: Option<House>,
}
