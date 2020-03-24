mod command;
mod dbus;
mod reader;
mod state;
mod tui;
mod unit;

use crate::command::*;
use serde_json::to_string as to_json;
use structopt::StructOpt;
use tokio_modbus::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let option = Option::from_args();

    let mut context = sync::tcp::connect(option.address)?;

    match &option.format {
        Format::Text => println!("{}", reader::read(&mut context)?),
        Format::Json => println!("{}", to_json(&reader::read(&mut context)?)?),
        Format::Tui => tui::run(&mut context)?,
    }

    Ok(())
}
