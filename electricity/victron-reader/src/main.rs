mod command;
mod dbus;
mod state;
mod unit;

use crate::{command::*, dbus::*, state::*, unit::*};
use serde_json::to_string as to_json;
use std::io;
use structopt::StructOpt;
use tokio_modbus::prelude::*;

fn read_holding_register(context: &mut sync::Context, address: u16) -> io::Result<u16> {
    Ok(context.read_holding_registers(address, 1)?[0])
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let option = Option::from_args();

    let socket_addr = option.address.parse()?;
    let mut context = sync::tcp::connect(socket_addr)?;

    let battery = {
        context.set_slave(Slave(DBUS_SERVICE_SYSTEM));

        let battery_state = read_holding_register(&mut context, BATTERY_STATE)?;
        let battery_power = read_holding_register(&mut context, BATTERY_POWER)?.to_watt();

        context.set_slave(Slave(DBUS_SERVICE_BATTERY));

        io::Result::<Battery>::Ok(Battery {
            state: match battery_state {
                0 => BatteryState::Idle,
                1 => BatteryState::Charging,
                2 => BatteryState::Discharging,
                _ => unreachable!("Unrecognized battery state (`{}`).", battery_state),
            },
            state_of_charge: read_holding_register(&mut context, BATTERY_STATE_OF_CHARGE)?
                .to_percent(),
            ongoing_power: battery_power,
            voltage: read_holding_register(&mut context, BATTERY_VOLTAGE)?.to_volt(),
            temperature: read_holding_register(&mut context, BATTERY_TEMPERATURE)?.to_degree(),
            health: read_holding_register(&mut context, BATTERY_STATE_OF_HEALTH)?.to_percent(),
        })
    }
    .ok();

    let pv_inverter = {
        context.set_slave(Slave(DBUS_SERVICE_PV_INVERTER));

        io::Result::<PvInverter>::Ok(PvInverter {
            l1: PvInverterPhase {
                voltage: read_holding_register(&mut context, PV_INVERTER_L1_VOLTAGE)?.to_volt(),
                current: read_holding_register(&mut context, PV_INVERTER_L1_CURRENT)?.to_amp(),
                power: read_holding_register(&mut context, PV_INVERTER_L1_POWER)?.to_watt(),
            },
            l2: PvInverterPhase {
                voltage: read_holding_register(&mut context, PV_INVERTER_L2_VOLTAGE)?.to_volt(),
                current: read_holding_register(&mut context, PV_INVERTER_L2_CURRENT)?.to_amp(),
                power: read_holding_register(&mut context, PV_INVERTER_L2_POWER)?.to_watt(),
            },
            l3: PvInverterPhase {
                voltage: read_holding_register(&mut context, PV_INVERTER_L3_VOLTAGE)?.to_volt(),
                current: read_holding_register(&mut context, PV_INVERTER_L3_CURRENT)?.to_amp(),
                power: read_holding_register(&mut context, PV_INVERTER_L3_POWER)?.to_watt(),
            },
        })
    }
    .ok();

    let vebus = {
        context.set_slave(Slave(DBUS_SERVICE_VEBUS));

        io::Result::<Vebus>::Ok(Vebus {
            frequency: read_holding_register(&mut context, VEBUS_OUTPUT_FREQUENCY)?.to_hertz(),
        })
    }
    .ok();

    let house = {
        context.set_slave(Slave(DBUS_SERVICE_SYSTEM));

        io::Result::<House>::Ok(House {
            l1: read_holding_register(&mut context, SYSTEM_AC_CONSUMPTION_L1)?.to_watt(),
            l2: read_holding_register(&mut context, SYSTEM_AC_CONSUMPTION_L2)?.to_watt(),
            l3: read_holding_register(&mut context, SYSTEM_AC_CONSUMPTION_L3)?.to_watt(),
        })
    }
    .ok();

    let state = State {
        battery,
        pv_inverter,
        vebus,
        house,
    };

    match &option.format {
        Format::Text => println!("{}", state),
        Format::Json => println!("{}", to_json(&state)?),
        _ => unimplemented!("Format `{}` not implemented yet.", option.format),
    }

    Ok(())
}
