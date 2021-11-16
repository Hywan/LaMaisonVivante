use std::net::SocketAddr;
use structopt::{clap::arg_enum, StructOpt};

/// This command allows to read values, or write new values to a Nilan
/// Compact P XL device.
#[derive(StructOpt, Debug)]
#[structopt(name = "nilan")]
pub struct Command {
    /// Modbus address of the Nilan,
    /// e.g. `192.168.1.142:502`. This option overwrites the value
    /// read from the configuration file.
    #[structopt(short = "a", long = "address")]
    pub address: Option<SocketAddr>,

    /// Print the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,

    /// The sub-command.
    #[structopt(subcommand)]
    pub kind: CommandKind,
}

#[derive(StructOpt, Debug)]
pub enum CommandKind {
    /// Read values from the Nilan.
    Read(ReadCommand),

    /// Write values to the Nilan.
    Write(WriteCommand),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "read")]
pub struct ReadCommand {
    /// Define the kind of outputs.
    #[structopt(
        short = "f",
        long = "format",
        possible_values = &ReadFormat::variants(),
        case_insensitive = true,
        default_value = "Text",
    )]
    pub format: ReadFormat,

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

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum ReadFormat {
        Text,
        Json,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "write")]
pub struct WriteCommand {
    /// Toggle the ventilation: turn on if disabled, or off if
    /// enabled.
    #[structopt(short = "v", long = "toggle-ventilation")]
    pub toggle_ventilation: bool,
}
