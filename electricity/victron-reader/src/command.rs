use std::net::SocketAddr;
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Format {
        Text,
        Json,
        Tui,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "victron-reader")]
pub struct Option {
    /// Modbus address of the Victron CCGX, e.g. `192.168.1.142:502`.
    #[structopt(short = "a", long = "address")]
    pub address: SocketAddr,

    /// Define the kind of outputs.
    #[structopt(
        short = "f",
        long = "format",
        possible_values = &Format::variants(),
        case_insensitive = true,
        default_value = "text",
    )]
    pub format: Format,
}
