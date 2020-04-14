use crate::{
    state::{Consumption, State},
    unit::*,
};
use regex::Regex;
use reqwest;
use std::{collections::HashMap, net::SocketAddr, str::FromStr};

pub async fn read(address: &SocketAddr) -> Result<State, Box<dyn std::error::Error>> {
    let total_consumption_url = format!(
        "http://{address}/hh?command=getCommand&value=cmdTotalverbrauch",
        address = address
    );
    let average_consumption_url = format!(
        "http://{address}/hh?command=getCommand&value=cmdDurchschnittverbrauch",
        address = address
    );

    let total_consumption = reqwest::get(&total_consumption_url);
    let average_consumption = reqwest::get(&average_consumption_url);

    let total_consumption = total_consumption
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let average_consumption = average_consumption
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let regex = Regex::new("(?P<kwh>[0-9,]+) kWh.+?(?P<l>[0-9]+) ℓ").unwrap();
    let captured = regex
        .captures(total_consumption.get("value").unwrap())
        .expect("Failed to capture the total consumption data.");

    let total_consumption = Consumption {
        power: Kwh(f64::from_str(&captured["kwh"].replace(",", "."))?),
        water: Liter(f64::from_str(&captured["l"])?),
    };

    let captured = regex
        .captures(average_consumption.get("value").unwrap())
        .expect("Failed to capture the average consumption data.");

    let average_consumption = Consumption {
        power: Kwh(f64::from_str(&captured["kwh"].replace(",", "."))?),
        water: Liter(f64::from_str(&captured["l"])?),
    };

    Ok(State {
        average_consumption,
        total_consumption,
    })
}
