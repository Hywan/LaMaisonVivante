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
            }),

            id => return Err(format!("Thing with ID `{}` cannot be identified", id)),
        })
    }
}

#[derive(Debug)]
pub struct Battery {
    ongoing_power: f64,
    temperature: f64,
    state_of_charge: f64,
    voltage: f64,
}

#[derive(Debug)]
pub struct PvInverter {
    voltage: f64,
    frequency: f64,
    power: f64,
    current: f64,
}

#[derive(Debug)]
pub struct HousePower {
    power: f64,
}
