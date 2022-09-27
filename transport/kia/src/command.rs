use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Format {
        Text,
        Json,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kia")]
pub struct Options {
    /// Username of the Kia Connect account.
    #[structopt(short = "u", long)]
    pub username: Option<String>,

    /// Password of the Kia Connect account.
    #[structopt(short = "p", long)]
    pub password: Option<String>,

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
