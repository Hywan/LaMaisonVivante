mod command;
mod configuration;

use crate::command::Options;
use human_panic::setup_panic;
use std::{io::prelude::*, net::TcpStream};
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

    let mut stream = TcpStream::connect(options.address.unwrap_or(configuration.address))?;

    println!("Sending a {:?} to {:?}…", options.action, options.subject);

    // The real piece of code.
    stream.write(&[options.subject as u8, b'\t', options.action as u8])?;

    Ok(())
}
