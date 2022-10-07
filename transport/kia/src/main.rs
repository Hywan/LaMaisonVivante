mod auth;
mod brand;
mod command;
mod configuration;
mod errors;
mod garage;
mod http;
mod identity;
mod thing;
mod units;
mod vehicles;

use crate::{
    auth::Authentification,
    brand::{Brand, Region},
    command::Options,
    errors::Error,
    garage::Garage,
};
use human_panic::setup_panic;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_panic!();

    let configuration_path = configuration::get_path()
        .map_err(ToOwned::to_owned)
        .map_err(Error::ReadConfiguration)?;
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
        options
            .password
            .or(configuration.password)
            .expect("No password provided for the Kia Connect account"),
    );

    if options.into_thing {
        thing::run(auth, options.thing_port.or(configuration.thing_port));
    } else {
        println!("Opening the garage…");

        let garage = Garage::new(Region::Europe, Brand::Kia, &auth).await?;

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
    }

    Ok(())
} // basic handler that responds with a static string
