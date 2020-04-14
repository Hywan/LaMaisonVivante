use crate::unit::*;
use serde::{de::Deserializer, Deserialize, Serialize};
use serde_json::Value;

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
struct Option {
    #[serde(deserialize_with = "parse_str_option")]
    set: bool,
}

fn parse_str_option<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|v: Value| match v {
        Value::Bool(v) => v,
        Value::String(string) => string != "none",
        _ => false,
    })
}

fn parse_option<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|o: Option| o.set)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Duration {
    #[serde(rename(deserialize = "act"))]
    remaining_seconds: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Program {
    Active {
        status: String,
        id: u32,
        name: String,
        duration: Duration,
        #[serde(rename(deserialize = "activeStepIndex"))]
        current_step: u32,
        #[serde(rename(deserialize = "stepIds"))]
        steps: Vec<u32>,
        #[serde(rename(deserialize = "eco"))]
        #[serde(deserialize_with = "parse_option")]
        eco: bool,
        #[serde(rename(deserialize = "steamfinish"))]
        #[serde(deserialize_with = "parse_option")]
        steam_finish: bool,
        #[serde(rename(deserialize = "partialload"))]
        #[serde(deserialize_with = "parse_option")]
        partial_load: bool,
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
