use confy;
use directories::ProjectDirs;
use human_panic::setup_panic;
use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    io::prelude::*,
    net::{SocketAddr, TcpStream},
    path::PathBuf,
};
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug)]
    #[repr(u8)]
    enum Subject {
        LaundryRoom ,
        Bathroom,
        LouiseBedroom,
        EliBedroom,
        Hall,
        LivingRoom,
        SittingRoom,
        DiningTable,
        KitchenIsland,
        Kitchen,
        ParentBed,
        ParentBathroom,
        ParentBedroom,
    }
}

arg_enum! {
    #[derive(Debug)]
    #[repr(u8)]
    enum Action {
        Pulse = 0,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "lights-controller")]
struct Options {
    /// Address of the Controllino; see `lights.ino` to see the port;
    /// e.g. `192.168.1.42:23`. This option overwrites the value read
    /// from the configuration file.
    #[structopt(short, long)]
    address: Option<SocketAddr>,

    /// Light to control.
    #[structopt(
        short,
        long,
        possible_values = &Subject::variants(),
        case_insensitive = true,
        default_value = "LivingRoom",
    )]
    subject: Subject,

    /// Type of signal/event to send on the light.
    #[structopt(
        short = "x",
        long,
        possible_values = &Action::variants(),
        case_insensitive = true,
        default_value = "Pulse",
    )]
    action: Action,

    /// Print the configuration path and exit.
    #[structopt(short = "c", long)]
    print_config_path: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    address: SocketAddr,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:23".parse().unwrap(),
        }
    }
}

fn configuration_path() -> Result<PathBuf, &'static str> {
    let name = "lights-controller";
    let project = ProjectDirs::from("rs", "", name)
        .ok_or_else(|| "Failed to find the configuration project directory.")?;

    let directory = project
        .config_dir()
        .to_str()
        .ok_or_else(|| "Failed to find the configuration directory.")?;

    let path: PathBuf = [directory, &format!("{}.toml", name)].iter().collect();

    Ok(path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();

    let configuration_path = configuration_path()?;
    let configuration: Configuration = confy::load_path(&configuration_path)?;

    let options = Options::from_args();

    if options.print_config_path {
        println!(
            "{}",
            configuration_path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|e| format!("{:?}", e))
        );

        return Ok(());
    }

    let mut stream = TcpStream::connect(options.address.unwrap_or(configuration.address))?;

    println!("Sending a {:?} to {:?}…", options.action, options.subject);

    // The real piece of code.
    stream.write(&[options.subject as u8, b'\t', options.action as u8])?;

    Ok(())
}
