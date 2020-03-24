use std::{
    io::prelude::*,
    net::{SocketAddr, TcpStream},
};
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug)]
    #[repr(u8)]
    enum Subject {
        LaundryRoom = 0,
        Bathroom = 1,
        LouiseBedroom = 2,
        EliBedroom = 3,
        Hall = 4,
        LivingRoom = 5,
        SittingRoom = 6,
        DiningTable = 7,
        KitchenIsland = 8,
        Kitchen = 9,
        ParentBed = 10,
        ParentBathroom = 11,
        ParentBedroom = 12,
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
    /// e.g. `192.168.1.42:23`.
    #[structopt(short, long)]
    address: SocketAddr,

    /// Light to control.
    #[structopt(
        short,
        long,
        possible_values = &Subject::variants(),
        case_insensitive = true
    )]
    subject: Subject,

    /// Type of signal/event to send on the light.
    #[structopt(
        short = "x",
        long,
        possible_values = &Action::variants(),
        case_insensitive = true,
        default_value = "pulse"
    )]
    action: Action,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let mut stream = TcpStream::connect(options.address)?;

    println!("Sending a {:} to {:?}…", options.action, options.subject);

    stream.write(&[options.subject as u8, b'\t', options.action as u8])?;

    Ok(())
}
