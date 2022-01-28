use crate::{
    command::AddressWithRefreshRate,
    database::{self, enums::AirState},
    thing::{generic, identified::*},
};
use diesel::{pg::PgConnection, prelude::*};
use std::{
    collections::HashMap,
    convert::TryInto,
    sync::mpsc::channel,
    thread,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
struct Message {
    things: Vec<generic::Thing>,
}

pub fn aggregate(addresses: Vec<AddressWithRefreshRate>, database_connection: PgConnection) {
    let (tx, rx) = channel();

    for AddressWithRefreshRate {
        address,
        refresh_rate,
    } in addresses.iter().cloned()
    {
        let tx = tx.clone();

        thread::spawn(move || loop {
            let mut things = reqwest::blocking::get(format!("http://{}", address))
                .unwrap()
                .json::<Vec<generic::Thing>>()
                .unwrap();

            for thing in things.iter_mut() {
                let property_values = reqwest::blocking::get(format!("{}/properties", thing.base))
                    .unwrap()
                    .json::<HashMap<String, generic::PropertyValue>>()
                    .unwrap();

                for (property_name, property_value) in thing.properties.iter_mut() {
                    if let Some(value) = property_values.get(property_name) {
                        property_value.value.replace(value.clone());
                    }
                }
            }

            tx.send(things).unwrap();

            thread::sleep(Duration::from_secs(refresh_rate.into()));
        });
    }

    loop {
        for _ in 0..addresses.len() {
            let message = rx
                .recv()
                .unwrap()
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<Thing>, _>>()
                .unwrap();
            //dbg!(&message);

            let now = SystemTime::now();

            let mut pv0 = None;
            let mut pv1 = None;
            let mut pv2 = None;
            let mut pv3 = None;

            for thing in message {
                match thing {
                    Thing::Battery(battery) => {
                        diesel::insert_into(database::schema::electricity_storage::table)
                            .values(&database::models::ElectricityStorage {
                                time: &now,
                                ongoing_power: battery.ongoing_power,
                                temperature: battery.temperature,
                                state_of_charge: battery.state_of_charge,
                                voltage: battery.voltage,
                            })
                            .execute(&database_connection)
                            .unwrap();
                    }

                    Thing::PvInverterAll(pv_inverter) => {
                        pv0.replace(pv_inverter);
                    }
                    Thing::PvInverter1(pv_inverter) => {
                        pv1.replace(pv_inverter);
                    }
                    Thing::PvInverter2(pv_inverter) => {
                        pv2.replace(pv_inverter);
                    }
                    Thing::PvInverter3(pv_inverter) => {
                        pv3.replace(pv_inverter);
                    }

                    Thing::HousePower(house_power) => {
                        diesel::insert_into(database::schema::electricity_consumption::table)
                            .values(&database::models::ElectricityConsumption {
                                time: &now,
                                house_power: house_power.power,
                                house_l1_power: house_power.l1_power,
                                house_l2_power: house_power.l2_power,
                                house_l3_power: house_power.l3_power,
                            })
                            .execute(&database_connection)
                            .unwrap();
                    }

                    Thing::DomesticHotWater(dhw) => {
                        diesel::insert_into(database::schema::domestic_hot_water::table)
                            .values(&database::models::DomesticHotWater {
                                time: &now,
                                top_of_the_tank_temperature: dhw.top_of_the_tank_temperature,
                                bottom_of_the_tank_temperature: dhw.bottom_of_the_tank_temperature,
                                wanted_temperature: dhw.wanted_temperature,
                            })
                            .execute(&database_connection)
                            .unwrap();
                    }

                    Thing::Air(air) => {
                        diesel::insert_into(database::schema::air::table)
                            .values(&database::models::Air {
                                time: &now,
                                state: match air.state.as_str() {
                                    "paused" => AirState::Paused,
                                    "running" => AirState::Running,
                                    v => panic!("Invalid `air.state` value, received `{:?}`", v),
                                },
                                inside_humidity: air.inside_humidity,
                                supplied_temperature_after_ground_coupled_heat_exchanger: air
                                    .supplied_temperature_after_ground_coupled_heat_exchanger,
                                supplied_temperature_after_heat_recovery_exchanger: air
                                    .supplied_temperature_after_heat_recovery_exchanger,
                                extracted_temperature: air.extracted_temperature,
                                discharged_temperature: air.discharged_temperature,
                                wanted_temperature: air.wanted_temperature,
                            })
                            .execute(&database_connection)
                            .unwrap();
                    }
                }
            }

            match (pv0, pv1, pv2, pv3) {
                (Some(pv0), Some(pv1), Some(pv2), Some(pv3)) => {
                    diesel::insert_into(database::schema::electricity_production::table)
                        .values(&database::models::ElectricityProduction {
                            time: &now,

                            l1_voltage: pv1.voltage,
                            l1_frequency: pv1.frequency,
                            l1_power: pv1.power,
                            l1_current: pv1.current,

                            l2_voltage: pv2.voltage,
                            l2_frequency: pv2.frequency,
                            l2_power: pv2.power,
                            l2_current: pv2.current,

                            l3_voltage: pv3.voltage,
                            l3_frequency: pv3.frequency,
                            l3_power: pv3.power,
                            l3_current: pv3.current,

                            voltage: pv0.voltage,
                            frequency: pv0.frequency,
                            power: pv0.power,
                            current: pv0.current,
                        })
                        .execute(&database_connection)
                        .unwrap();
                }

                _ => (),
            }
        }
    }
}
