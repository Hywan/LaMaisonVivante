mod command;
mod configuration;
mod modbus;
mod reader;
mod state;
mod thing;
mod unit;

use crate::command::*;
use human_panic::setup_panic;
use serde_json::to_string as to_json;
use structopt::StructOpt;
use tokio_modbus::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();

    let configuration_path = configuration::get_path()?;
    let configuration = configuration::load(&configuration_path)?;

    let command = Command::from_args();

    if command.print_config_path {
        println!(
            "{}",
            configuration_path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|e| format!("{:?}", e))
        );

        return Ok(());
    }

    match command.kind {
        CommandKind::Read(read_command) => {
            if read_command.into_thing {
                thing::run(
                    command.address.unwrap_or(configuration.address),
                    read_command.thing_port.or(configuration.thing_port),
                );
            } else {
                let mut context =
                    client::sync::tcp::connect(command.address.unwrap_or(configuration.address))?;

                match &read_command.format {
                    ReadFormat::Text => println!("{:#?}", reader::read(&mut context)?),
                    ReadFormat::Json => println!("{}", to_json(&reader::read(&mut context)?)?),
                }
            }
        }

        CommandKind::Write(write_command) => {
            let mut context =
                client::sync::tcp::connect(command.address.unwrap_or(configuration.address))?;

            let ventilation = reader::read_ventilation(&mut context)?;

            use crate::{modbus::*, state::*};

            if dbg!(write_command.toggle_ventilation) {
                match dbg!(ventilation.activity) {
                    VentilationActivity::Off => {
                        dbg!(context.write_single_register(VENTILATION_ACTIVITY, 1)?);
                    }

                    VentilationActivity::On => {
                        dbg!(context.write_single_register(VENTILATION_ACTIVITY, 0)?);
                    }
                }
            }
        }
    }

    Ok(())
}
