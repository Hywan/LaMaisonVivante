use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kia")]
pub struct Options {
    /// Username of the Kia Connect account.
    #[structopt(short = "u", long)]
    pub username: Option<String>,

    /// Password of the Kia Connect account.
    #[structopt(short = "s", long)]
    pub password: Option<String>,

    /// Turns this program into a Thing, i.e. a new Web of Things
    /// device.
    #[structopt(short = "t", long)]
    pub into_thing: bool,

    /// Port of the Thing. Requires `--into-thing` to be
    /// effective. This option overwrites the value read from the
    /// configuration file.
    #[structopt(short = "p", long)]
    pub thing_port: Option<u16>,

    /// Print the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}
