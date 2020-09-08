use std::{
    io::{self, BufRead, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    dbg!(buffer);

    Ok(())
}

pub fn start_listening(address: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        match handle_client(stream?) {
            Ok(_) => continue,  // nothing to do
            Err(_) => continue, // fail silently
        }
    }

    Ok(())
}
