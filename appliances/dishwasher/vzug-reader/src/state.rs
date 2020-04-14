use crate::unit::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub model: String,
    pub description: String,
    #[serde(rename(deserialize = "serialNumber"))]
    pub serial_number: String,
    #[serde(rename(deserialize = "articleNumber"))]
    pub article_number: String,
    #[serde(rename(deserialize = "apiVersion"))]
    pub api_version: String,
}

#[derive(Debug, Serialize)]
pub struct Consumption {
    pub power: Kwh,
    pub water: Liter,
}

#[derive(Debug, Serialize)]
pub struct State {
    pub device: Device,
    pub average_consumption: Consumption,
    pub total_consumption: Consumption,
}
