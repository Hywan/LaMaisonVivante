use std::net::SocketAddr;
use structopt::{clap::arg_enum, StructOpt};

/// This command allows to read values, or write new values to an
/// Alfen NG9xx charging station.
#[derive(StructOpt, Debug)]
#[structopt(name = "alfen")]
pub struct Command {
    /// Modbus address of the Alfen,
    /// e.g. `192.168.1.142:502`. This option overwrites the value
    /// read from the configuration file.
    #[structopt(short = "a", long = "address")]
    pub address: Option<SocketAddr>,

    /// Print the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,

    /// The sub-command.
    #[structopt(subcommand)]
    pub kind: Option<CommandKind>,
}

#[derive(StructOpt, Debug)]
pub enum CommandKind {
    /// Read values from the Alfen.
    Read(ReadCommand),
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
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum ReadFormat {
        Text,
        Json,
    }
}
