use tokio_modbus::prelude::*;

const DBUS_SERVICE_SYSTEM: u8 = 100;
const DBUS_SERVICE_BATTERY: u8 = 225;
const DBUS_SERVICE_PV_INVERTER: u8 = 20;
const DBUS_SERVICE_VEBUS: u8 = 246;

const BATTERY_POWER: u16 = 842;
const BATTERY_STATE: u16 = 844;
const BATTERY_VOLTAGE: u16 = 259;
const BATTERY_TEMPERATURE: u16 = 262;
const BATTERY_STATE_OF_CHARGE: u16 = 266;
const BATTERY_STATE_OF_HEALTH: u16 = 304;

const PV_INVERTER_L1_VOLTAGE: u16 = 1027;
const PV_INVERTER_L1_CURRENT: u16 = 1028;
const PV_INVERTER_L1_POWER: u16 = 1029;
const PV_INVERTER_L2_VOLTAGE: u16 = 1031;
const PV_INVERTER_L2_CURRENT: u16 = 1032;
const PV_INVERTER_L2_POWER: u16 = 1033;
const PV_INVERTER_L3_VOLTAGE: u16 = 1035;
const PV_INVERTER_L3_CURRENT: u16 = 1036;
const PV_INVERTER_L3_POWER: u16 = 1037;

const VEBUS_OUTPUT_FREQUENCY: u16 = 21;
const VEBUS_OUTPUT_POWER_1: u16 = 23;
const VEBUS_OUTPUT_POWER_2: u16 = 24;
const VEBUS_OUTPUT_POWER_3: u16 = 25;

/*
battery 272 -> low-SoC alarm (value = 2 for an alarm, 0 otherwise)
battery 274 -> high temperature alarm (value = 2 for an alarm, 0 otherwise)
*/

trait Unit {
    fn to_percent(&self) -> f32;
    fn to_volt(&self) -> f32;
    fn to_amp(&self) -> f32;
    fn to_watt(&self) -> f32;
    fn to_kwh(&self) -> f32;
    fn to_degree(&self) -> f32;
    fn to_hertz(&self) -> f32;
}

impl Unit for u16 {
    fn to_percent(&self) -> f32 {
        (*self as f32) / 10.0
    }

    fn to_volt(&self) -> f32 {
        (*self as f32) / 100.0
    }

    fn to_amp(&self) -> f32 {
        (*self as f32)
    }

    fn to_watt(&self) -> f32 {
        (*self as f32)
    }

    fn to_kwh(&self) -> f32 {
        (*self as f32)
    }

    fn to_degree(&self) -> f32 {
        (*self as f32) / 10.0
    }

    fn to_hertz(&self) -> f32 {
        (*self as f32) / 100.0
    }
}

fn read_holding_register(context: &mut sync::Context, address: u16) -> u16 {
    let buffer = context
        .read_holding_registers(address, 1)
        .expect("Failed to read holding registers.");

    buffer[0]
}

pub fn main() {
    let socket_addr = "192.168.1.117:502"
        .parse()
        .expect("Failed to parse the socket address.");
    let mut context = sync::tcp::connect(socket_addr).expect("Failed to connect to the server.");

    context.set_slave(Slave(DBUS_SERVICE_SYSTEM));

    let battery_state = read_holding_register(&mut context, BATTERY_STATE);
    let battery_power = read_holding_register(&mut context, BATTERY_POWER).to_watt();

    context.set_slave(Slave(DBUS_SERVICE_BATTERY));

    println!(
        "Battery:
    battery_state: {state}
    state of charge: {soc}%
    ongoing power: {power}W
    voltage: {voltage}V
    temperature: {temperature}°C
    state of health: {health}%",
        state = match battery_state {
            0 => "idle",
            1 => "charging",
            2 => "discharging",
            _ => unreachable!(),
        },
        soc = read_holding_register(&mut context, BATTERY_STATE_OF_CHARGE).to_percent(),
        power = battery_power,
        voltage = read_holding_register(&mut context, BATTERY_VOLTAGE).to_volt(),
        temperature = read_holding_register(&mut context, BATTERY_TEMPERATURE).to_degree(),
        health = read_holding_register(&mut context, BATTERY_STATE_OF_HEALTH).to_percent(),
    );

    context.set_slave(Slave(DBUS_SERVICE_PV_INVERTER));

    let l1_power = read_holding_register(&mut context, PV_INVERTER_L1_POWER).to_watt();
    let l2_power = read_holding_register(&mut context, PV_INVERTER_L2_POWER).to_watt();
    let l3_power = read_holding_register(&mut context, PV_INVERTER_L3_POWER).to_watt();

    println!(
        "\nPV inverter:
    L1 voltage: {l1_voltage}V
    L1 current: {l1_current}A
    L1 power: {l1_power}W

    L2 voltage: {l2_voltage}V
    L2 current: {l2_current}A
    L2 power: {l2_power}W

    L3 voltage: {l3_voltage}V
    L3 current: {l3_current}A
    L3 power: {l3_power}W

    total power: {power}W",
        l1_voltage = read_holding_register(&mut context, PV_INVERTER_L1_VOLTAGE).to_volt(),
        l1_current = read_holding_register(&mut context, PV_INVERTER_L1_CURRENT).to_amp(),
        l1_power = l1_power,
        l2_voltage = read_holding_register(&mut context, PV_INVERTER_L2_VOLTAGE).to_volt(),
        l2_current = read_holding_register(&mut context, PV_INVERTER_L2_CURRENT).to_amp(),
        l2_power = l2_power,
        l3_voltage = read_holding_register(&mut context, PV_INVERTER_L3_VOLTAGE).to_volt(),
        l3_current = read_holding_register(&mut context, PV_INVERTER_L3_CURRENT).to_amp(),
        l3_power = l3_power,
        power = l1_power + l2_power + l3_power,
    );

    context.set_slave(Slave(DBUS_SERVICE_VEBUS));

    println!(
        "\nVebus:
    output frequency: {frequency}Hz
    output power 1: {power1}W
    output power 2: {power2}W
    output power 3: {power3}W",
        frequency = read_holding_register(&mut context, VEBUS_OUTPUT_FREQUENCY).to_hertz(),
        power1 = read_holding_register(&mut context, VEBUS_OUTPUT_POWER_1).to_watt(),
        power2 = read_holding_register(&mut context, VEBUS_OUTPUT_POWER_2).to_watt(),
        power3 = read_holding_register(&mut context, VEBUS_OUTPUT_POWER_3).to_watt(),
    );

    println!(
        "\nHouse:
    power: {power}W",
        power = (l1_power + l2_power + l3_power) - battery_power,
    );
}
