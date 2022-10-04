mod auth;
mod brand;
mod command;
mod configuration;
mod errors;
mod garage;
mod http;
mod identity;
mod units;
mod vehicles;

use crate::{
    auth::Authentification,
    brand::{Brand, Region},
    command::Options,
    errors::Error,
    garage::Garage,
};
use axum::{extract::path::Path, http::StatusCode, response::Json, routing::get, Router};
use human_panic::setup_panic;
use serde_json::json;
use std::net::SocketAddr;
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

    println!("Opening the garage…");

    let garage = Garage::new(Region::Europe, Brand::Kia, &auth).await?;

    println!("Looking for vehicles…");

    let vehicles = garage.vehicles().await?;

    match options.server_port {
        None => {
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

        Some(server_port) => {
            let router = Router::new().route(
                "/:vehicle",
                get(|Path(vehicle_vin): Path<String>| async move {
                    match vehicles.iter().find_map(|vehicle| {
                        if vehicle.vin == vehicle_vin {
                            Some(vehicle)
                        } else {
                            None
                        }
                    }) {
                        Some(vehicle) => (
                            StatusCode::OK,
                            Json(Some(json!({
                                "description": vehicle,
                                "state": vehicle.state().await.unwrap(),
                            }))),
                        ),
                        None => (StatusCode::NOT_FOUND, Json(None)),
                    }
                }),
            );

            let server_address = SocketAddr::from(([127, 0, 0, 1], server_port));
            println!("Listening on {}", server_address);

            axum::Server::bind(&server_address)
                .serve(router.into_make_service())
                .await
                .unwrap();
        }
    }

    Ok(())
} // basic handler that responds with a static string
