use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kia")]
pub struct Options {
    /// Username of the Kia Connect account.
    #[structopt(short = "u", long)]
    pub username: Option<String>,

    /// Password of the Kia Connect account.
    #[structopt(short = "p", long)]
    pub password: Option<String>,

    /// Port for the server.
    #[structopt(short = "P", long)]
    pub server_port: Option<u16>,

    /// Print the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}
