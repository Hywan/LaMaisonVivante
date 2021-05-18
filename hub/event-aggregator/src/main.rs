mod aggregator;
mod command;
mod configuration;
mod database;
mod thing;

#[macro_use]
extern crate diesel;

use crate::command::Options;
use diesel::{pg::PgConnection, prelude::*};
use human_panic::setup_panic;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();

    let configuration_path = configuration::get_path()?;
    let configuration = configuration::load(&configuration_path)?;

    let mut options = Options::from_args();

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

    let addresses = &mut options.addresses;
    addresses.extend(configuration.addresses.into_iter().by_ref());

    let database_url = options.database_url.unwrap_or(configuration.database_url);

    if database_url.is_empty() {
        panic!(
            "The database URL is empty, use `--database-url` or the configuration file to set it"
        );
    }

    let database_connection = PgConnection::establish(&database_url).expect(&format!(
        "Failed to connect to database at `{}`",
        database_url
    ));

    aggregator::aggregate(addresses.to_vec(), database_connection);

    Ok(())
}
