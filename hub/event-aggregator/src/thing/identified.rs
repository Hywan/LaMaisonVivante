use crate::thing::generic;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Thing {
    Battery(Battery),
    PvInverterAll(PvInverter),
    PvInverter1(PvInverter),
    PvInverter2(PvInverter),
    PvInverter3(PvInverter),
    HousePower(HousePower),
    DomesticHotWater(DomesticHotWater),
    Air(Air),
}

impl TryFrom<&generic::Thing> for Thing {
    type Error = String;

    fn try_from(generic: &generic::Thing) -> Result<Self, Self::Error> {
        macro_rules! property {
            ($thing_name:ident . $name:ident from $generic_thing:ident $as_ty:ident) => {
                $generic_thing
                    .properties
                    .get(stringify!($name))
                    .ok_or_else(|| {
                        concat!(
                            "Property `",
                            stringify!($name),
                            "` of the `Thing::",
                            stringify!($thing_name),
                            "` is missing"
                        )
                    })?
                    .value
                    .as_ref()
                    .ok_or_else(|| {
                        concat!(
                            "Property value `",
                            stringify!($name),
                            "` of the `Thing::",
                            stringify!($thing_name),
                            "` is missing"
                        )
                    })?
                    .$as_ty()
                    .ok_or_else(|| {
                        concat!(
                            "Property `",
                            stringify!($name),
                            "` failed to be read with `",
                            stringify!($as_ty),
                            "`"
                        )
                    })?
            };
        }

        Ok(match generic.id.as_str() {
            "urn:dev:ops:battery" => Thing::Battery(Battery {
                ongoing_power: property!(Battery.ongoing_power from generic as_f64),
                temperature: property!(Battery.temperature from generic as_f64),
                state_of_charge: property!(Battery.state_of_charge from generic as_f64),
                voltage: property!(Battery.voltage from generic as_f64),
            }),

            "urn:dev:ops:pv-inverter-0" => Thing::PvInverterAll(PvInverter {
                current: property!(PvInverter.current from generic as_f64),
                power: property!(PvInverter.power from generic as_f64),
                voltage: property!(PvInverter.voltage from generic as_f64),
                frequency: property!(PvInverter.frequency from generic as_f64),
            }),

            "urn:dev:ops:pv-inverter-1" => Thing::PvInverter1(PvInverter {
                current: property!(PvInverter.current from generic as_f64),
                power: property!(PvInverter.power from generic as_f64),
                voltage: property!(PvInverter.voltage from generic as_f64),
                frequency: property!(PvInverter.frequency from generic as_f64),
            }),

            "urn:dev:ops:pv-inverter-2" => Thing::PvInverter2(PvInverter {
                current: property!(PvInverter.current from generic as_f64),
                power: property!(PvInverter.power from generic as_f64),
                voltage: property!(PvInverter.voltage from generic as_f64),
                frequency: property!(PvInverter.frequency from generic as_f64),
            }),

            "urn:dev:ops:pv-inverter-3" => Thing::PvInverter3(PvInverter {
                current: property!(PvInverter.current from generic as_f64),
                power: property!(PvInverter.power from generic as_f64),
                voltage: property!(PvInverter.voltage from generic as_f64),
                frequency: property!(PvInverter.frequency from generic as_f64),
            }),

            "urn:dev:ops:house-power" => Thing::HousePower(HousePower {
                power: property!(HousePower.power from generic as_f64),
                l1_power: property!(HousePower.l1_power from generic as_f64),
                l2_power: property!(HousePower.l2_power from generic as_f64),
                l3_power: property!(HousePower.l3_power from generic as_f64),
            }),

            "urn:dev:ops:domestic-hot-water" => Thing::DomesticHotWater(DomesticHotWater {
                top_of_the_tank_temperature: property!(DomesticHotWater.top_of_the_tank from generic as_f64),
                bottom_of_the_tank_temperature: property!(DomesticHotWater.bottom_of_the_tank from generic as_f64),
                wanted_temperature: property!(DomesticHotWater.wanted from generic as_f64),
            }),

            "urn:dev:ops:ventilation" => Thing::Air(Air {
                state: property!(Air.state from generic as_str).to_string(),
                inside_humidity: property!(Air.inside_air_humidity from generic as_f64),
                supplied_temperature_after_ground_coupled_heat_exchanger: property!(Air.supplied_air_after_ground_coupled_heat_exchanger from generic as_f64),
                supplied_temperature_after_heat_recovery_exchanger: property!(Air.supplied_air_after_heat_recovery_exchanger from generic as_f64),
                extracted_temperature: property!(Air.extracted_air from generic as_f64),
                discharged_temperature: property!(Air.discharged_air from generic as_f64),
                wanted_temperature: property!(Air.wanted_air_inside from generic as_f64),
            }),

            id => return Err(format!("Thing with ID `{}` cannot be identified", id)),
        })
    }
}

#[derive(Debug)]
pub struct Battery {
    pub ongoing_power: f64,
    pub temperature: f64,
    pub state_of_charge: f64,
    pub voltage: f64,
}

#[derive(Debug)]
pub struct PvInverter {
    pub voltage: f64,
    pub frequency: f64,
    pub power: f64,
    pub current: f64,
}

#[derive(Debug)]
pub struct HousePower {
    pub power: f64,
    pub l1_power: f64,
    pub l2_power: f64,
    pub l3_power: f64,
}

#[derive(Debug)]
pub struct DomesticHotWater {
    pub top_of_the_tank_temperature: f64,
    pub bottom_of_the_tank_temperature: f64,
    pub wanted_temperature: f64,
}

#[derive(Debug)]
pub struct Air {
    pub state: String,
    pub inside_humidity: f64,
    pub supplied_temperature_after_ground_coupled_heat_exchanger: f64,
    pub supplied_temperature_after_heat_recovery_exchanger: f64,
    pub extracted_temperature: f64,
    pub discharged_temperature: f64,
    pub wanted_temperature: f64,
}
