mod command;
mod configuration;
mod thing;
mod writer;

use crate::command::Options;
use human_panic::setup_panic;
use std::net::TcpStream;
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

    let address = options.address.unwrap_or(configuration.address);

    if options.into_thing {
        thing::run(address, options.thing_port);
    } else {
        println!("Sending a {:?} to {:?}…", options.action, options.subject);

        let stream = TcpStream::connect(address)?;
        writer::send(&stream, options.subject, options.action)?;
    }

    Ok(())
}
