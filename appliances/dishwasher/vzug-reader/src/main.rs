mod command;
mod configuration;
mod state;
mod unit;

use crate::{command::{Format, Options}, state::{Consumption, State}};
use human_panic::setup_panic;
use regex::Regex;
use reqwest;
use serde_json::to_string as to_json;
use std::{collections::HashMap, str::FromStr};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();

    let configuration_path = configuration::get_path()?;
    let configuration = configuration::load(&configuration_path)?;

    let options = Options::from_args();

    if options.print_config_path {
        println!(
            "{}",
            configuration_path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|e| format!("{:?}", e))
        );

        return Ok(());
    }

    let address = options.address.unwrap_or(configuration.address);

    let total_consumption_url = format!("http://{address}/hh?command=getCommand&value=cmdTotalverbrauch", address = address);
    let average_consumption_url = format!("http://{address}/hh?command=getCommand&value=cmdDurchschnittverbrauch", address = address);

    let total_consumption = reqwest::get(&total_consumption_url).await?.json::<HashMap<String, String>>().await?;
    let average_consumption = reqwest::get(&average_consumption_url).await?.json::<HashMap<String, String>>().await?;

    dbg!(&average_consumption);

    let regex = Regex::new("(?P<kwh>[0-9,]+) kWh.+?(?P<l>[0-9]+) ℓ").unwrap();
    let captured = regex.captures(total_consumption.get("value").unwrap()).expect("Failed to capture the total consumption data.");

    let total_consumption = Consumption {
        power: f64::from_str(&captured["kwh"].replace(",", "."))?,
        water: f64::from_str(&captured["l"])?,
    };

    let captured = regex.captures(average_consumption.get("value").unwrap()).expect("Failed to capture the average consumption data.");

    let average_consumption = Consumption {
        power: f64::from_str(&captured["kwh"].replace(",", "."))?,
        water: f64::from_str(&captured["l"])?,
    };

    let state = State {
        average: average_consumption,
        total: total_consumption,
    };

    match &options.format {
        Format::Text => println!("{:#?}", state),
        Format::Json => println!("{}", to_json(&state)?),
    }

    Ok(())
}
