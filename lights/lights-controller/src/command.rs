use std::net::SocketAddr;
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    pub enum Subject {
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

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Subject {
    fn to_string(&self) -> String {
        match self {
            Self::LaundryRoom => "Buanderie",
            Self::Bathroom => "Salle de bain",
            Self::LouiseBedroom => "Chambre Louise",
            Self::EliBedroom => "Chambre Éli",
            Self::Hall => "Entrée",
            Self::LivingRoom => "Espace de vie",
            Self::SittingRoom => "Canapé",
            Self::DiningTable => "Table à manger",
            Self::KitchenIsland => "Îlot",
            Self::Kitchen => "Cuisine",
            Self::ParentBed => "Lit parental",
            Self::ParentBathroom => "Salle de bain parents",
            Self::ParentBedroom => "Suite parentale",
        }
        .to_string()
    }
}

arg_enum! {
    #[derive(Debug)]
    #[repr(u8)]
    pub enum Action {
        Pulse = 0,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "lights-controller")]
pub struct Options {
    /// Address of the Controllino; see `lights.ino` to see the port;
    /// e.g. `192.168.1.42:23`. This option overwrites the value read
    /// from the configuration file.
    #[structopt(short, long)]
    pub address: Option<SocketAddr>,

    /// Light to control.
    #[structopt(
        short,
        long,
        possible_values = &Subject::variants(),
        case_insensitive = true,
        default_value = "LivingRoom",
    )]
    pub subject: Subject,

    /// Type of signal/event to send on the light.
    #[structopt(
        short = "x",
        long,
        possible_values = &Action::variants(),
        case_insensitive = true,
        default_value = "Pulse",
    )]
    pub action: Action,

    /// Prints the configuration path and exit.
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
