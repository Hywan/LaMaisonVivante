use std::net::SocketAddr;
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Format {
        Text,
        Json,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "vzug-reader")]
pub struct Options {
    /// HTTP address of the VZug adora diswhwasher,
    /// e.g. `192.168.1.142:80`. This option overwrites the value
    /// read from the configuration file.
    #[structopt(short = "a", long = "address")]
    pub address: Option<SocketAddr>,

    /// Define the kind of outputs.
    #[structopt(
        short = "f",
        long = "format",
        possible_values = &Format::variants(),
        case_insensitive = true,
        default_value = "Text",
    )]
    pub format: Format,

    /// Print the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}
