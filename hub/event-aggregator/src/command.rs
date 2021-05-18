use serde::{Deserialize, Serialize};
use std::{
    net::{AddrParseError, SocketAddr},
    num::{NonZeroU64, ParseIntError},
    str::FromStr,
};
use structopt::StructOpt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AddressWithRefreshRate {
    pub address: SocketAddr,
    pub refresh_rate: NonZeroU64,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "hub-event-aggregator")]
pub struct Options {
    /// Addresses to listen to, from which to collect and aggregate
    /// events, paired with their refresh rates, separated by a `@`,
    /// e.g. `localhost:1234@10`.
    #[structopt(short = "a", long)]
    pub addresses: Vec<AddressWithRefreshRate>,

    /// The database URL.
    #[structopt(short = "d", long)]
    pub database_url: Option<String>,

    /// Prints the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,
}

impl FromStr for AddressWithRefreshRate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (address, refresh_rate) = s.split_once('@').ok_or_else(|| format!("Address and refresh rate must be separated by an `@`, e.g. `localhost:1234@10`, given `{}`", s))?;

        Ok(AddressWithRefreshRate {
            address: address.parse().map_err(|e: AddrParseError| e.to_string())?,
            refresh_rate: refresh_rate
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        })
    }
}
