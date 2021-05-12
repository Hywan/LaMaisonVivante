mod aggregator;
mod command;
mod configuration;
mod thing;

use crate::command::Options;
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
    addresses.extend(&configuration.addresses);

    let refresh_rate = options.refresh_rate.unwrap_or(configuration.refresh_rate);

    aggregator::aggregate(addresses.to_vec(), refresh_rate);

    Ok(())
}
