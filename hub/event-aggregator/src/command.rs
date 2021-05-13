use std::{net::SocketAddr, num::NonZeroU64};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hub-event-aggregator")]
pub struct Options {
    /// Addresses to listen to, from which to collect and aggregate
    /// events.
    #[structopt(short = "a", long)]
    pub addresses: Vec<SocketAddr>,

    /// Refresh rate (in seconds).
    #[structopt(short = "r", long)]
    pub refresh_rate: Option<NonZeroU64>,

    /// The database URL.
    #[structopt(short = "d", long)]
    pub database_url: Option<String>,

    /// Prints the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}
