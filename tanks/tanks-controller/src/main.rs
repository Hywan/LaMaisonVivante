use std::{
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: TcpStream) -> io::Result<()> {
    dbg!(&stream);
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    dbg!(buffer);

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:1234")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
