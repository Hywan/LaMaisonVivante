use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hub-event-automator")]
pub struct Options {
    /// The database URL.
    #[structopt(short = "d", long)]
    pub database_url: Option<String>,

    /// WebThing URL for the blinds.
    #[structopt(short = "b", long)]
    pub blinds_url: Option<SocketAddr>,

    /// Prints the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}
