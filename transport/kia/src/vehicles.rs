use crate::{
    brand::{Brand, BrandConfiguration},
    errors::Error,
    http::Client,
    identity::Tokens,
    units::*,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fmt;
use std::time::Duration;

const VEHICLES_URL: &'static str = "/api/v1/spa/vehicles";

#[derive(Debug, Clone)]
pub struct Vehicles {
    vehicles: Vec<Vehicle>,
}

impl Vehicles {
    pub async fn new(
        brand: Brand,
        brand_configuration: &BrandConfiguration,
        tokens: &Tokens,
    ) -> Result<Vehicles, Error> {
        let mut http_request_headers = reqwest::header::HeaderMap::with_capacity(3);
        http_request_headers.insert("Authorization", tokens.access_token.parse().unwrap());
        http_request_headers.insert("ccsp-service-id", brand.client_id().parse().unwrap());
        http_request_headers.insert(
            "ccsp-application-id",
            brand.application_id().parse().unwrap(),
        );
        http_request_headers.insert("ccsp-device-id", tokens.device_id.parse().unwrap());
        http_request_headers.insert("Stamp", tokens.stamp.parse().unwrap());

        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename(deserialize = "resMsg"))]
            result_message: ResponseVehicles,
        }

        #[derive(Debug, Deserialize)]
        pub struct ResponseVehicles {
            vehicles: Vec<ResponseVehicle>,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseVehicle {
            vin: String,
            #[serde(rename(deserialize = "vehicleId"))]
            vehicle_id: String,
            #[serde(rename(deserialize = "vehicleName"))]
            vehicle_name: String,
            nickname: String,
            master: bool,
            #[serde(rename(deserialize = "carShare"))]
            car_share: u32,
        }

        Ok(Self {
            vehicles: Client::get(format!(
                "{url}{path}",
                url = brand_configuration.uri,
                path = VEHICLES_URL,
            ))?
            .headers(http_request_headers)
            .send()
            .await
            .map_err(Error::Http)?
            .json::<Response>()
            .await
            .map_err(Error::Http)?
            .result_message
            .vehicles
            .into_iter()
            .map(
                |ResponseVehicle {
                     vin,
                     vehicle_id,
                     vehicle_name,
                     nickname,
                     master,
                     car_share,
                 }| {
                    Vehicle {
                        brand,
                        brand_configuration: brand_configuration.clone(),
                        tokens: tokens.clone(),
                        vin,
                        vehicle_id,
                        vehicle_name,
                        nickname,
                        master,
                        car_share,
                    }
                },
            )
            .collect(),
        })
    }

    #[allow(unused)]
    pub fn get(&self, index: usize) -> Option<&Vehicle> {
        self.vehicles.get(index)
    }

    pub fn len(&self) -> usize {
        self.vehicles.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vehicle> {
        self.vehicles.iter()
    }
}

#[derive(Clone, Serialize)]
pub struct Vehicle {
    #[serde(skip)]
    tokens: Tokens,

    #[serde(skip)]
    brand: Brand,

    #[serde(skip)]
    brand_configuration: BrandConfiguration,

    pub vin: String,
    pub vehicle_id: String,
    pub vehicle_name: String,
    pub nickname: String,
    pub master: bool,
    pub car_share: u32,
}

impl fmt::Debug for Vehicle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Vehicle")
            .field("vin", &self.vin)
            .field("vehicle_id", &self.vehicle_id)
            .field("vehicle_name", &self.vehicle_name)
            .field("nickame", &self.nickname)
            .field("master", &self.master)
            .field("car_share", &self.car_share)
            .finish_non_exhaustive()
    }
}

impl Vehicle {
    pub async fn state(&self) -> Result<State, Error> {
        let mut http_request_headers = reqwest::header::HeaderMap::with_capacity(3);
        http_request_headers.insert("Authorization", self.tokens.access_token.parse().unwrap());
        http_request_headers.insert("ccsp-service-id", self.brand.client_id().parse().unwrap());
        http_request_headers.insert(
            "ccsp-application-id",
            self.brand.application_id().parse().unwrap(),
        );
        http_request_headers.insert("ccsp-device-id", self.tokens.device_id.parse().unwrap());
        http_request_headers.insert("Stamp", self.tokens.stamp.parse().unwrap());

        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename(deserialize = "resMsg"))]
            result_message: ResponseVehicleState,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseVehicleState {
            #[serde(rename(deserialize = "vehicleStatusInfo"))]
            vehicle_state: State,
        }

        Ok(Client::get(format!(
            "{url}{path}/{vehicle_id}/status/latest",
            url = self.brand_configuration.uri,
            path = VEHICLES_URL,
            vehicle_id = self.vehicle_id,
        ))?
        .headers(http_request_headers)
        .send()
        .await
        .map_err(Error::Http)?
        .json::<Response>()
        .await
        .map_err(Error::Http)?
        .result_message
        .vehicle_state)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    #[serde(rename(deserialize = "vehicleStatus"))]
    pub status: Status,

    #[serde(rename(deserialize = "vehicleLocation"))]
    pub location: Location,

    #[serde(rename(deserialize = "odometer"), deserialize_with = "distance_to_km")]
    pub odometer: Kilometer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename(deserialize = "evStatus"))]
    pub battery: Battery,

    #[serde(rename(deserialize = "doorOpen"))]
    pub doors: Doors,

    #[serde(rename(deserialize = "windowOpen"))]
    pub windows: Windows,

    #[serde(
        rename(deserialize = "airTemp"),
        deserialize_with = "temperature_to_celcius"
    )]
    pub targeted_temperature: Celcius,

    #[serde(rename(deserialize = "airCtrlOn"))]
    pub is_air_conditionning_enabled: bool,

    #[serde(rename(deserialize = "engine"))]
    pub is_engine_running: bool,

    #[serde(rename(deserialize = "doorLock"))]
    pub is_locked: bool,

    #[serde(rename(deserialize = "trunkOpen"))]
    pub is_trunk_opened: bool,

    #[serde(rename(deserialize = "hoodOpen"))]
    pub is_frunk_opened: bool,

    #[serde(rename(deserialize = "defrost"))]
    pub is_defrost_enabled: bool,

    #[serde(
        rename(deserialize = "steerWheelHeat"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_steer_wheel_heat_enabled: bool,

    #[serde(
        rename(deserialize = "sideBackWindowHeat"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_side_back_window_heat_enabled: bool,

    #[serde(
        rename(deserialize = "hazardStatus"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_hazard_detected: bool,

    #[serde(rename(deserialize = "smartKeyBatteryWarning"))]
    pub has_smart_key_battery_issue: bool,

    #[serde(rename(deserialize = "washerFluidStatus"))]
    pub has_washer_fluid_issue: bool,

    #[serde(rename(deserialize = "breakOilStatus"))]
    pub has_break_oil_issue: bool,

    #[serde(
        rename(deserialize = "tailLampStatus"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub has_tail_lamp_issue: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Doors {
    #[serde(
        rename(deserialize = "frontLeft"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_front_left_opened: bool,

    #[serde(
        rename(deserialize = "frontRight"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_front_right_opened: bool,

    #[serde(
        rename(deserialize = "backLeft"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_back_left_opened: bool,

    #[serde(
        rename(deserialize = "backRight"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_back_right_opened: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Windows {
    #[serde(
        rename(deserialize = "frontLeft"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_front_left_opened: bool,

    #[serde(
        rename(deserialize = "frontRight"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_front_right_opened: bool,

    #[serde(
        rename(deserialize = "backLeft"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_back_left_opened: bool,

    #[serde(
        rename(deserialize = "backRight"),
        deserialize_with = "deserialize_int_to_bool"
    )]
    pub is_back_right_opened: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Battery {
    #[serde(rename(deserialize = "batteryCharge"))]
    pub is_charging: bool,

    #[serde(rename(deserialize = "batteryStatus"))]
    pub state_of_charge: Percent,

    #[serde(
        rename(deserialize = "drvDistance"),
        deserialize_with = "deserialize_range"
    )]
    pub remaining_range: u32,

    #[serde(
        rename(deserialize = "remainTime2"),
        deserialize_with = "deserialize_estimated_charging_duration"
    )]
    pub estimated_charging_duration: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename(deserialize = "coord"))]
    pub coordinates: Coordinates,

    #[serde(rename(deserialize = "accuracy"))]
    pub precision_dilution: Option<PrecisionDilution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(rename(deserialize = "lat"))]
    pub latitude: Coordinate,

    #[serde(rename(deserialize = "lon"))]
    pub longitude: Coordinate,

    #[serde(rename(deserialize = "alt"))]
    pub altitude: Option<Meter>,
}

/// [DOP] (Dilution of precision).
///
/// [DOP]: https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation)
#[derive(Debug, Serialize, Deserialize)]
pub struct PrecisionDilution {
    #[serde(rename(deserialize = "hdop"))]
    pub horizontal: u32,

    #[serde(rename(deserialize = "pdop"))]
    pub position: u32,
}

fn deserialize_int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i32 = de::Deserialize::deserialize(deserializer)?;

    Ok(value != 0)
}

fn deserialize_range<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = de::Deserialize::deserialize(deserializer)?;
    let path: &'static str = "/0/rangeByFuel/totalAvailableRange/value";

    Ok(match value.pointer(path) {
        Some(Value::Number(number)) if number.is_u64() => number.as_u64().unwrap() as u32,

        Some(_) => {
            return Err(de::Error::invalid_value(
                de::Unexpected::Other("a number that is not a `u64`"),
                &"a `u64`",
            ))
        }

        None => return Err(de::Error::missing_field(path))?,
    })
}

fn deserialize_estimated_charging_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct PairValue {
        value: u64,
    }

    #[derive(Deserialize)]
    struct RemainingTime {
        #[serde(rename(deserialize = "atc"))]
        estimated_current_charging_duration: PairValue,
    }

    let remaining_time: RemainingTime = de::Deserialize::deserialize(deserializer)?;

    Ok(Duration::from_secs(
        remaining_time.estimated_current_charging_duration.value * 60,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename(deserialize = "resMsg"))]
            result_message: ResponseVehicleState,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseVehicleState {
            #[serde(rename(deserialize = "vehicleStatusInfo"))]
            vehicle_state: State,
        }

        let text = include_str!("test.json");

        let j: Response = serde_json::from_str(text).unwrap();
        dbg!(j.result_message.vehicle_state);
    }
}
