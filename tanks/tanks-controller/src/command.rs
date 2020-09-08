use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tanks-controller")]
pub struct Options {
    /// Address the server listens to; e.g. `192.168.1.42:1234`. This
    /// option overwrites the value read from the configuration file.
    #[structopt(short, long)]
    pub address: Option<SocketAddr>,

    /// Prints the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}
