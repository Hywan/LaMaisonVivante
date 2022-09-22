use crate::{modbus::*, state::*, unit::*};
use std::io::Result;
use tokio_modbus::prelude::*;

pub fn set_socket_current(context: &mut sync::Context, state: &State, current: u16) -> Result<()> {
    let max_current = state.station_status.max_current;
    let current = f32::from(current);

    if Amp(current) >= max_current {
        panic!(
            "New current value ({:?}) must be lower than {:?}",
            current, max_current
        );
    }

    let bits = current.to_bits();
    let bytes = unsafe { std::mem::transmute::<_, [u16; 2]>(bits) };
    let value = [bytes[1], bytes[0]];

    context.set_slave(Slave(SINGLE_SOCKET_SLAVE));
    context.write_multiple_registers(SOCKET_MAX_CURRENT, &value[..])?;

    Ok(())
}
