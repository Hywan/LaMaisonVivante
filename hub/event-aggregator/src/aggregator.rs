use crate::thing::{
    generic::{PropertyValue, Thing},
    identified,
};
use std::{
    collections::HashMap, convert::TryInto, net::SocketAddr, num::NonZeroU64, sync::mpsc::channel,
    thread, time::Duration,
};

#[derive(Debug)]
struct Message {
    things: Vec<Thing>,
}

pub fn aggregate(addresses: Vec<SocketAddr>, refresh_rate: NonZeroU64) {
    let (tx, rx) = channel();

    for address in addresses.iter().cloned() {
        let tx = tx.clone();

        thread::spawn(move || loop {
            let mut things = reqwest::blocking::get(format!("http://{}", address))
                .unwrap()
                .json::<Vec<Thing>>()
                .unwrap();

            for thing in things.iter_mut() {
                let property_values = reqwest::blocking::get(format!("{}/properties", thing.base))
                    .unwrap()
                    .json::<HashMap<String, PropertyValue>>()
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
                .collect::<Result<Vec<identified::Thing>, _>>()
                .unwrap();
            dbg!(message);
        }
    }
}
