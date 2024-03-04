mod actions;
mod command;
mod configuration;
mod database;
mod event_loop;
mod events;
mod state;

#[macro_use]
extern crate diesel;

use crate::command::Options;
use diesel::prelude::*;
use human_panic::setup_panic;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let database_url = {
        let database_url = options.database_url.unwrap_or(configuration.database_url);

        if database_url.is_empty() {
            panic!(
                "The database URL is empty, use `--database-url` or the configuration file to set it"
            );
        }

        database_url
    };

    let blinds_url = options.blinds_url.unwrap_or(configuration.blinds_url);

    let database_connection = PgConnection::establish(&database_url).expect(&format!(
        "Failed to connect to database at `{}`",
        &database_url
    ));

    event_loop::run(database_connection, &blinds_url);

    Ok(())
}
