use crate::{
    database,
    thing::{generic, identified::*},
};
use diesel::{pg::PgConnection, prelude::*};
use std::{
    collections::HashMap,
    convert::TryInto,
    net::SocketAddr,
    num::NonZeroU64,
    sync::mpsc::channel,
    thread,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
struct Message {
    things: Vec<generic::Thing>,
}

pub fn aggregate(
    addresses: Vec<SocketAddr>,
    refresh_rate: NonZeroU64,
    database_connection: PgConnection,
) {
    let (tx, rx) = channel();

    for address in addresses.iter().cloned() {
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
            dbg!(&message);

            let now = SystemTime::now();

            for thing in message {
                match thing {
                    Thing::Battery(battery) => {
                        let new_battery = database::models::ElectricityStorage {
                            time: &now,
                            ongoing_power: battery.ongoing_power,
                            temperature: battery.temperature,
                            state_of_charge: battery.state_of_charge,
                            voltage: battery.voltage,
                        };

                        diesel::insert_into(database::schema::electricity_storage::table)
                            .values(&new_battery)
                            .execute(&database_connection)
                            .unwrap();
                    }
                    _ => (),
                }
            }
        }
    }
}
