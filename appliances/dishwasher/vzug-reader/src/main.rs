mod command;
mod configuration;
mod reader;
mod state;
mod thing;
mod unit;

use crate::command::{Format, Options};
use human_panic::setup_panic;
use serde_json::to_string as to_json;
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

    if options.into_thing {
        thing::run(address, options.thing_port);
    } else {
        let state = reader::read(&address).await?;

        match &options.format {
            Format::Text => println!("{:#?}", state),
            Format::Json => println!("{}", to_json(&state)?),
        }
    }

    Ok(())
}
