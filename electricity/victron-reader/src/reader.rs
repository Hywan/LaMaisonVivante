use crate::{dbus::*, state::*, unit::*};
use std::io::Result;
use tokio_modbus::prelude::*;

fn read_holding_register(context: &mut sync::Context, address: u16) -> Result<u16> {
    Ok(context.read_holding_registers(address, 1)?[0])
}

fn read_battery(mut context: &mut sync::Context) -> Result<Battery> {
    context.set_slave(Slave(DBUS_SERVICE_SYSTEM));

    let battery_state = match read_holding_register(&mut context, BATTERY_STATE)? {
        0 => BatteryState::Idle,
        1 => BatteryState::Charging,
        2 => BatteryState::Discharging,
        v @ _ => unreachable!("Unrecognized battery state (`{}`).", v),
    };
    let battery_power = read_holding_register(&mut context, BATTERY_POWER)?;

    context.set_slave(Slave(DBUS_SERVICE_BATTERY));

    Ok(Battery {
        ongoing_power: match &battery_state {
            BatteryState::Idle => 0u16.to_watt(),
            BatteryState::Charging => battery_power.to_watt(),
            BatteryState::Discharging => (-((1 << 16) - battery_power as i32)).to_watt(),
        },
        state: battery_state,
        state_of_charge: read_holding_register(&mut context, BATTERY_STATE_OF_CHARGE)?.to_percent(),
        voltage: read_holding_register(&mut context, BATTERY_VOLTAGE)?.to_volt(),
        temperature: read_holding_register(&mut context, BATTERY_TEMPERATURE)?.to_degree(),
        health: read_holding_register(&mut context, BATTERY_STATE_OF_HEALTH)?.to_percent(),
    })
}

fn read_pv_inverter(mut context: &mut sync::Context) -> Result<PvInverter> {
    context.set_slave(Slave(DBUS_SERVICE_PV_INVERTER));

    Ok(PvInverter {
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

fn read_vebus(mut context: &mut sync::Context) -> Result<Vebus> {
    context.set_slave(Slave(DBUS_SERVICE_VEBUS));

    Ok(Vebus {
        frequency: read_holding_register(&mut context, VEBUS_OUTPUT_FREQUENCY)?.to_hertz(),
    })
}

fn read_house(mut context: &mut sync::Context) -> Result<House> {
    context.set_slave(Slave(DBUS_SERVICE_SYSTEM));

    Ok(House {
        l1: read_holding_register(&mut context, SYSTEM_AC_CONSUMPTION_L1)?.to_watt(),
        l2: read_holding_register(&mut context, SYSTEM_AC_CONSUMPTION_L2)?.to_watt(),
        l3: read_holding_register(&mut context, SYSTEM_AC_CONSUMPTION_L3)?.to_watt(),
    })
}

pub fn read(mut context: &mut sync::Context) -> Result<State> {
    Ok(State {
        battery: read_battery(&mut context).ok(),
        pv_inverter: read_pv_inverter(&mut context).ok(),
        vebus: read_vebus(&mut context).ok(),
        house: read_house(&mut context).ok(),
    })
}
