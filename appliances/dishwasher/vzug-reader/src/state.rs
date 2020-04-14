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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Option<T> {
    set: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Duration {
    act: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Program {
    Active {
        status: String,
        id: u32,
        name: String,
        #[serde(rename(deserialize = "activeStepIndex"))]
        current_step: u32,
        #[serde(rename(deserialize = "stepIds"))]
        steps: Vec<u32>,
        #[serde(rename(deserialize = "eco"))]
        echo_option: Option<String>,
        #[serde(rename(deserialize = "steamfinish"))]
        steam_finish: Option<bool>,
        #[serde(rename(deserialize = "partialload"))]
        partial_load: Option<bool>,
    },
    Idle {
        status: String,
    },
}

#[derive(Debug, Serialize)]
pub struct State {
    pub device: Device,
    pub average_consumption: Consumption,
    pub total_consumption: Consumption,
    pub current_program: Program,
}
