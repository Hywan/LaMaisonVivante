mod dbus;
mod state;
mod unit;

use crate::{dbus::*, state::*, unit::*};
use std::io::Result;
use tokio_modbus::prelude::*;

/*
battery 272 -> low-SoC alarm (value = 2 for an alarm, 0 otherwise)
battery 274 -> high temperature alarm (value = 2 for an alarm, 0 otherwise)
*/

fn read_holding_register(context: &mut sync::Context, address: u16) -> Result<u16> {
    Ok(context.read_holding_registers(address, 1)?[0])
}

pub fn main() -> Result<()> {
    let socket_addr = "192.168.1.117:502"
        .parse()
        .expect("Failed to parse the socket address.");
    let mut context = sync::tcp::connect(socket_addr)?;

    let battery = {
        context.set_slave(Slave(DBUS_SERVICE_SYSTEM));

        let battery_state = read_holding_register(&mut context, BATTERY_STATE)?;
        let battery_power = read_holding_register(&mut context, BATTERY_POWER)?.to_watt();

        context.set_slave(Slave(DBUS_SERVICE_BATTERY));

        Result::<Battery>::Ok(Battery {
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

        Result::<PvInverter>::Ok(PvInverter {
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

        Result::<Vebus>::Ok(Vebus {
            frequency: read_holding_register(&mut context, VEBUS_OUTPUT_FREQUENCY)?.to_hertz(),
        })
    }
    .ok();

    let state = State {
        battery,
        pv_inverter,
        vebus,
    };

    println!("{}", state);

    /*
    println!(
        "\nHouse:
    power: {power}W",
        power = Watt((l1_power.0 + l2_power.0 + l3_power.0) - battery_power.0),
    );
    */

    Ok(())
}
