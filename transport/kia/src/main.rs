mod command;
mod configuration;
mod auth;
mod brand;
mod errors;
mod garage;
mod http;
mod identity;
mod units;
mod vehicles;

use crate::{
    auth::Authentification,
    brand::{Brand, Region},
    errors::Error,
    garage::Garage,
    command::Options,
};
use human_panic::setup_panic;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_panic!();

    let configuration_path = configuration::get_path().map_err(ToOwned::to_owned).map_err(Error::ReadConfiguration)?;
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

    let auth = Authentification::new(
        options.username.unwrap_or(configuration.username),
        options.password.unwrap_or(configuration.password),
    );

    let brand = Brand::Kia;

    println!("Opening the garage…");

    let garage = Garage::new(Region::Europe, brand, &auth).await?;

    println!("Looking for vehicles…");

    let vehicles = garage.vehicles().await?;
    let number_of_vehicles = vehicles.len();

    println!("Found {} vehicle(s).", number_of_vehicles);

    for vehicle in vehicles.iter() {
        println!(
            "\n## {nickname} ({vin})\n\n{vehicle:#?}\n\nwith state:\n\n{state:#?}",
            nickname = vehicle.nickname,
            vin = vehicle.vin,
            vehicle = vehicle,
            state = vehicle.state().await?
        );
    }

    Ok(())
}
