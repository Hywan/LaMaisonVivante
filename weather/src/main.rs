mod command;
mod configuration;
mod reader;
mod state;
mod thing;

use crate::command::Options;
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

    let openweathermap_api_key = options
        .openweathermap_api_key
        .unwrap_or(configuration.openweathermap_api_key);

    if options.into_thing {
        thing::run(
            &openweathermap_api_key,
            options.thing_port.or(configuration.thing_port),
        );
    } else {
        let state = reader::read(&openweathermap_api_key).unwrap();

        println!("{:#?}", state);
    }

    Ok(())
}
