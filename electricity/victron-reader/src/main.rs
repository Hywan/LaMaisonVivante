use tokio_modbus::prelude::*;

pub fn main() {
    let socket_addr = "192.168.1.117:502"
        .parse()
        .expect("Failed to parse the socket address.");
    let unit = 100;
    let mut ctx = sync::tcp::connect(socket_addr).expect("Failed to connect to the server.");
    ctx.set_slave(Slave(unit));

    let address = 843;
    let count = 1;

    let buff = ctx
        .read_holding_registers(address, count)
        .expect("Failed to read holding registers");

    println!("Response is '{:?}'", buff);
}
