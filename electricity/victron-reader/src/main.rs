mod command;
mod configuration;
mod dbus;
mod reader;
mod state;
mod tui;
mod unit;

use crate::command::*;
use human_panic::setup_panic;
use serde_json::to_string as to_json;
use structopt::StructOpt;
use tokio_modbus::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut context = sync::tcp::connect(options.address.unwrap_or(configuration.address))?;

    match &options.format {
        Format::Text => println!("{}", reader::read(&mut context)?),
        Format::Json => println!("{}", to_json(&reader::read(&mut context)?)?),
        Format::Tui => tui::run(&mut context)?,
    }

    Ok(())
}
