use crate::command::{Action, Subject};
use std::{
    io::{self, prelude::*},
    net::TcpStream,
};

pub fn send(mut stream: &TcpStream, subject: Subject, action: Action) -> io::Result<usize> {
    stream.write(&[subject as u8, b'\t', action as u8])
}
