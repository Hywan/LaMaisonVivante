use std::net::SocketAddr;
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    pub enum Subject {
        Kitchen,
        LivingRoom,
        ParentBedroom,
        EliBedroom,
        LouiseBedroom,
        Bathroom,
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Subject {
    fn to_string(&self) -> String {
        match self {
            Self::Kitchen => "Cuisine",
            Self::LivingRoom => "Espace de vie",
            Self::ParentBedroom => "Suite parentale",
            Self::EliBedroom => "Chambre Éli",
            Self::LouiseBedroom => "Chambre Louise",
            Self::Bathroom => "Salle de bain",
        }
        .to_string()
    }
}

arg_enum! {
    #[derive(Debug)]
    #[repr(u8)]
    pub enum Action {
        Unmoving = 0,
        MovingUp = 1,
        MovingDown = 2,
        Opening = 3,
        Closing = 4
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "blinds-controller")]
pub struct Options {
    /// Address of the Controllino; see `blinds.ino` to see the port;
    /// e.g. `192.168.1.42:23`. This option overwrites the value read
    /// from the configuration file.
    #[structopt(short, long)]
    pub address: Option<SocketAddr>,

    /// Blind to control.
    #[structopt(
        short,
        long,
        possible_values = &Subject::variants(),
        case_insensitive = true,
        default_value = "LivingRoom",
    )]
    pub subject: Subject,

    /// Type of signal/event to send on the blind.
    #[structopt(
        short = "x",
        long,
        possible_values = &Action::variants(),
        case_insensitive = true,
        default_value = "Opening",
    )]
    pub action: Action,

    /// Prints the configuration path and exit.
    #[structopt(short = "c", long)]
    pub print_config_path: bool,

    /// Turns this program into a Thing, i.e. a new Web of Things
    /// device.
    #[structopt(short = "t", long)]
    pub into_thing: bool,

    /// Port of the Thing. Requires `--into-thing` to be effective.
    #[structopt(short = "p", long)]
    pub thing_port: Option<u16>,
}