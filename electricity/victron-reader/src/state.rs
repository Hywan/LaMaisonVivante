use crate::unit::*;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub enum BatteryState {
    Idle,
    Discharging,
    Charging,
}

#[derive(Serialize)]
pub struct Battery {
    pub state: BatteryState,
    pub state_of_charge: Percent,
    pub ongoing_power: Watt,
    pub voltage: Volt,
    pub temperature: Degree,
    pub health: Percent,
}

#[derive(Serialize)]
pub struct PvInverterPhase {
    pub voltage: Volt,
    pub current: Amp,
    pub power: Watt,
}

#[derive(Serialize)]
pub struct PvInverter {
    pub l1: PvInverterPhase,
    pub l2: PvInverterPhase,
    pub l3: PvInverterPhase,
}

#[derive(Serialize)]
pub struct Vebus {
    pub frequency: Hertz,
}

#[derive(Serialize)]
pub struct House {
    pub power: Watt,
}

#[derive(Serialize)]
pub struct State {
    pub battery: Option<Battery>,
    pub pv_inverter: Option<PvInverter>,
    pub vebus: Option<Vebus>,
    pub house: Option<House>,
}

impl fmt::Display for BatteryState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Idle => "idle",
                Self::Discharging => "discharging",
                Self::Charging => "charging",
            }
        )
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Battery {{
    state: {state},
    state of charge: {soc},
    ongoing power: {power},
    voltage: {voltage},
    temperature: {temperature},
    health: {health},
}}",
            state = self.state,
            soc = self.state_of_charge,
            power = self.ongoing_power,
            voltage = self.voltage,
            temperature = self.temperature,
            health = self.health,
        )
    }
}

impl fmt::Display for PvInverter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "PvInverter {{
    L1 voltage: {l1_voltage},
    L1 current: {l1_current},
    L1 power: {l1_power},
    L2 voltage: {l2_voltage},
    L2 current: {l2_current},
    L2 power: {l2_power},
    L3 voltage: {l3_voltage},
    L3 current: {l3_current},
    L3 power: {l3_power},
    total power: {power},
}}",
            l1_voltage = self.l1.voltage,
            l1_current = self.l1.current,
            l1_power = self.l1.power,
            l2_voltage = self.l2.voltage,
            l2_current = self.l2.current,
            l2_power = self.l2.power,
            l3_voltage = self.l3.voltage,
            l3_current = self.l3.current,
            l3_power = self.l3.power,
            power = self.l1.power + self.l2.power + self.l3.power,
        )
    }
}

impl fmt::Display for Vebus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Vebus {{
    frequency: {frequency}
}}",
            frequency = self.frequency,
        )
    }
}

impl fmt::Display for House {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "House {{
    power: {power}
}}",
            power = self.power,
        )
    }
}

impl fmt::Display for State {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{battery}

{pv_inverter}

{vebus}

{house}",
            battery = match &self.battery {
                Some(battery) => battery.to_string(),
                None => "None".to_string(),
            },
            pv_inverter = match &self.pv_inverter {
                Some(pv_inverter) => pv_inverter.to_string(),
                None => "None".to_string(),
            },
            vebus = match &self.vebus {
                Some(vebus) => vebus.to_string(),
                None => "None".to_string(),
            },
            house = match &self.house {
                Some(house) => house.to_string(),
                None => "None".to_string(),
            },
        )
    }
}
