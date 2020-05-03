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
pub struct Options {
    /// Modbus address of the Victron CCGX,
    /// e.g. `192.168.1.142:502`. This option overwrites the value
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

    /// Turns this program into a Thing, i.e. a new Web of Things
    /// device.
    #[structopt(short = "t", long)]
    pub into_thing: bool,

    /// Port of the Thing. Requires `--into-thing` to be
    /// effective. This option overwrites the value read from the
    /// configuration file.
    #[structopt(short = "p", long)]
    pub thing_port: Option<u16>,
}
