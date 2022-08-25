use crate::unit::*;
use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct StationInformation {
    pub name: String,
    pub manufacturer: String,
    pub platform_type: String,
    pub serial_number: String,
    pub firmware_version: String,
    pub date: DateTime<FixedOffset>,
    pub uptime_ms: u64,
}

#[derive(Debug, Serialize, Default)]
pub struct StationStatus {
    pub max_current: Amp,
    pub temperature: Degree,
    pub is_ocpp_connected: bool,
    pub number_of_sockets: u16,
}

#[derive(Debug, Serialize, Default)]
pub struct SocketPhase {
    pub voltage: Volt,
    pub current: Amp,
}

#[derive(Debug, Serialize, Default)]
pub struct Socket {
    pub availability: SocketAvailability,
    pub status: SocketStatus,
    pub number_of_phases: PhaseNumber,
    pub l1: SocketPhase,
    pub l2: SocketPhase,
    pub l3: SocketPhase,
    pub power: Watt,
    pub frequency: Hertz,
    pub real_power_delivered: WattHour,
    pub session: SocketSession,
}

#[derive(Debug, Serialize, Default)]
pub enum PhaseNumber {
    #[default]
    Unknown,
    One,
    Three,
}

#[derive(Debug, Serialize, Default)]
pub enum SocketAvailability {
    #[default]
    Unknown,
    Inoperative,
    Operative,
}

#[derive(Debug, Serialize, Default)]
pub struct SocketStatus {
    pub pwm_signal: bool,
    pub connected: bool,
    pub charging: bool,
    pub is_error: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct SocketSession {
    pub max_current: Amp,
    pub actual_applied_max_current: Amp,
    pub remaining_time_before_fallback_to_safe_current: u32,
}

#[derive(Debug, Serialize, Default)]
pub struct State {
    pub station_information: StationInformation,
    pub station_status: StationStatus,
    pub socket: Socket,
}