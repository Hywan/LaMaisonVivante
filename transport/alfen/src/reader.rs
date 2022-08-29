use crate::{modbus::*, state::*, unit::*};
use chrono::offset::{FixedOffset, TimeZone};
use std::{cmp, io::Result, time::Duration};
use tokio_modbus::prelude::*;

trait RegistryReader {
    type Target;

    fn number_of_registers() -> usize;
    fn to_target(bytes: &[u16]) -> Self::Target;
}

impl RegistryReader for u16 {
    type Target = Self;

    fn number_of_registers() -> usize {
        1
    }

    fn to_target(bytes: &[u16]) -> Self::Target {
        bytes[0]
    }
}

impl RegistryReader for i16 {
    type Target = Self;

    fn number_of_registers() -> usize {
        1
    }

    fn to_target(bytes: &[u16]) -> Self::Target {
        bytes[0] as _
    }
}

impl RegistryReader for u32 {
    type Target = Self;

    fn number_of_registers() -> usize {
        2
    }

    fn to_target(bytes: &[u16]) -> Self::Target {
        let (prefix, bytes, suffix) = unsafe { bytes.align_to::<u8>() };

        assert!(prefix.is_empty());
        assert!(suffix.is_empty());

        Self::from_be_bytes(bytes[..Self::number_of_registers() * 2].try_into().unwrap())
    }
}

impl RegistryReader for u64 {
    type Target = Self;

    fn number_of_registers() -> usize {
        4
    }

    fn to_target(bytes: &[u16]) -> Self::Target {
        let (prefix, bytes, suffix) = unsafe { bytes.align_to::<u8>() };

        assert!(prefix.is_empty());
        assert!(suffix.is_empty());

        Self::from_be_bytes(bytes[..Self::number_of_registers() * 2].try_into().unwrap())
    }
}

impl RegistryReader for f32 {
    type Target = Self;

    fn number_of_registers() -> usize {
        2
    }

    fn to_target(bytes: &[u16]) -> Self::Target {
        f32::from_bits(unsafe { std::mem::transmute::<_, u32>([bytes[1], bytes[0]]) })
    }
}

struct FixedString<const N: usize>;

impl<const N: usize> RegistryReader for FixedString<N> {
    type Target = String;

    fn number_of_registers() -> usize {
        N
    }

    fn to_target(bytes: &[u16]) -> Self::Target {
        let (prefix, bytes, suffix) = unsafe { bytes.align_to::<u8>() };

        assert!(prefix.is_empty());
        assert!(suffix.is_empty());

        let bytes = bytes
            .chunks_exact(2)
            .flat_map(|bytes| [bytes[1], bytes[0]])
            .collect::<Vec<u8>>();

        let max = bytes.len();
        let last = bytes.iter().position(|x| *x == b'\0').unwrap_or(max);

        String::from_utf8_lossy(&bytes[..cmp::min(max, last)]).to_string()
    }
}

fn read_holding_register<R>(context: &mut sync::Context, address: u16) -> Result<R::Target>
where
    R: RegistryReader,
{
    let registers = context.read_holding_registers(address, R::number_of_registers() as _)?;

    Ok(R::to_target(&registers))
}

fn read_station_information(context: &mut sync::Context) -> Result<StationInformation> {
    context.set_slave(Slave(PRODUCT_IDENTIFICATION_SLAVE));

    Ok(StationInformation {
        name: read_holding_register::<FixedString<17>>(context, NAME)?,
        manufacturer: read_holding_register::<FixedString<5>>(context, MANUFACTURER)?,
        platform_type: read_holding_register::<FixedString<17>>(context, PLATFORM_TYPE)?,
        serial_number: read_holding_register::<FixedString<11>>(context, STATION_SERIAL_NUMBER)?,
        firmware_version: read_holding_register::<FixedString<17>>(context, FIRMWARE_VERSION)?,
        date: FixedOffset::east(read_holding_register::<i16>(context, TIMEZONE)?.into())
            .ymd(
                read_holding_register::<i16>(context, DATE_YEAR)?
                    .try_into()
                    .unwrap(),
                read_holding_register::<i16>(context, DATE_MONTH)?
                    .try_into()
                    .unwrap(),
                read_holding_register::<i16>(context, DATE_DAY)?
                    .try_into()
                    .unwrap(),
            )
            .and_hms(
                read_holding_register::<i16>(context, TIME_HOUR)?
                    .try_into()
                    .unwrap(),
                read_holding_register::<i16>(context, TIME_MINUTE)?
                    .try_into()
                    .unwrap(),
                read_holding_register::<i16>(context, TIME_SECOND)?
                    .try_into()
                    .unwrap(),
            ),
        uptime: Duration::from_millis(read_holding_register::<u64>(context, UPTIME)?),
    })
}

fn read_station_status(context: &mut sync::Context) -> Result<StationStatus> {
    context.set_slave(Slave(STATION_STATUS_SLAVE));

    Ok(StationStatus {
        max_current: read_holding_register::<f32>(context, STATION_ACTIVE_MAX_CURRENT)?.to_amp(),
        temperature: read_holding_register::<f32>(context, TEMPERATURE)?.to_degree(),
        is_ocpp_connected: matches!(read_holding_register::<u16>(context, OCPP_STATE)?, 1),
        number_of_sockets: read_holding_register::<u16>(context, NUMBER_OF_SOCKETS)?,
    })
}

fn read_socket(context: &mut sync::Context) -> Result<Socket> {
    context.set_slave(Slave(SINGLE_SOCKET_SLAVE));

    Ok(Socket {
        availability: match read_holding_register::<u16>(context, SOCKET_AVAILABILITY)? {
            0 => SocketAvailability::Inoperative,
            1 => SocketAvailability::Operative,
            _ => SocketAvailability::Unknown,
        },
        status: match read_holding_register::<FixedString<5>>(context, SOCKET_STATUS)?.as_str() {
            "B1" | "C1" | "D1" => SocketStatus::Connected { pwm_signal: false },
            "B2" => SocketStatus::Connected { pwm_signal: true },
            "C2" | "D2" => SocketStatus::Charging,
            "A" | "E" => SocketStatus::Disconnected,
            "F" => SocketStatus::Error,
            _ => SocketStatus::Unknown,
        },
        number_of_phases: match read_holding_register::<u16>(context, SOCKET_NUMBER_OF_PHASES)? {
            1 => PhaseNumber::One,
            3 => PhaseNumber::Three,
            _ => PhaseNumber::Unknown,
        },
        l1: SocketPhase {
            voltage: read_holding_register::<f32>(context, SOCKET_L1_VOLTAGE)?.to_volt(),
            current: read_holding_register::<f32>(context, SOCKET_L1_CURRENT)?.to_amp(),
        },
        l2: SocketPhase {
            voltage: read_holding_register::<f32>(context, SOCKET_L2_VOLTAGE)?.to_volt(),
            current: read_holding_register::<f32>(context, SOCKET_L2_CURRENT)?.to_amp(),
        },
        l3: SocketPhase {
            voltage: read_holding_register::<f32>(context, SOCKET_L3_VOLTAGE)?.to_volt(),
            current: read_holding_register::<f32>(context, SOCKET_L3_CURRENT)?.to_amp(),
        },
        power: read_holding_register::<f32>(context, SOCKET_POWER_SUM)?.to_watt(),
        frequency: read_holding_register::<f32>(context, SOCKET_FREQUENCY)?.to_hertz(),
        session: SocketSession {
            max_current: read_holding_register::<f32>(context, SOCKET_MAX_CURRENT)?.to_amp(),
            actual_applied_max_current: read_holding_register::<f32>(
                context,
                SOCKET_ACTUAL_APPLIED_MAX_CURRENT,
            )?
            .to_amp(),
            remaining_time_before_fallback_to_safe_current: read_holding_register::<u32>(
                context,
                SOCKET_MAX_CURRENT_VALID_TIME,
            )?,
        },
    })
}

pub fn read(context: &mut sync::Context) -> Result<State> {
    Ok(State {
        station_information: read_station_information(context)?,
        station_status: read_station_status(context)?,
        socket: read_socket(context)?,
    })
}
