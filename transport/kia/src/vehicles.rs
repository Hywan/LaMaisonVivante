use crate::{
    brand::{Brand, BrandConfiguration},
    errors::Error,
    http::Client,
    identity::Tokens,
};
use serde::{de, Deserialize, Deserializer};
use std::fmt;

const VEHICLES_URL: &'static str = "/api/v1/spa/vehicles";

#[derive(Debug)]
pub struct Vehicles<'a> {
    vehicles: Vec<Vehicle<'a>>,
}

impl<'a> Vehicles<'a> {
    pub async fn new(
        brand: Brand,
        brand_configuration: &'a BrandConfiguration,
        tokens: &'a Tokens,
    ) -> Result<Vehicles<'a>, Error> {
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
            #[serde(rename = "resMsg")]
            result_message: ResponseVehicles,
        }

        #[derive(Debug, Deserialize)]
        pub struct ResponseVehicles {
            vehicles: Vec<ResponseVehicle>,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseVehicle {
            vin: String,
            #[serde(rename = "vehicleId")]
            vehicle_id: String,
            #[serde(rename = "vehicleName")]
            vehicle_name: String,
            nickname: String,
            master: bool,
            #[serde(rename = "carShare")]
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
                        brand_configuration,
                        tokens,
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

    pub fn get(&self, index: usize) -> Option<&Vehicle> {
        self.vehicles.get(index)
    }
}

pub struct Vehicle<'a> {
    tokens: &'a Tokens,
    brand: Brand,
    brand_configuration: &'a BrandConfiguration,

    pub vin: String,
    pub vehicle_id: String,
    pub vehicle_name: String,
    pub nickname: String,
    pub master: bool,
    pub car_share: u32,
}

impl<'a> fmt::Debug for Vehicle<'a> {
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

impl<'a> Vehicle<'a> {
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
            #[serde(rename = "resMsg")]
            result_message: ResponseVehicleState,
        }

        #[derive(Debug, Deserialize)]
        struct ResponseVehicleState {
            #[serde(rename = "vehicleStatusInfo")]
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

#[derive(Debug, Deserialize)]
pub struct State {
    #[serde(rename = "vehicleStatus")]
    pub status: Status,

    #[serde(rename = "vehicleLocation")]
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    #[serde(rename = "airCtrlOn")]
    pub air_climate_enabled: bool,

    #[serde(rename = "engine")]
    pub engine_enabled: bool,

    #[serde(rename = "doorLock")]
    pub doors_locked: bool,

    #[serde(rename = "doorOpen")]
    pub doors: Doors,

    #[serde(rename = "trunkOpen")]
    pub trunk_opened: bool,

    #[serde(rename = "hoodOpen")]
    pub frunk_opened: bool,

    #[serde(rename = "evStatus")]
    pub battery: Battery,
}

#[derive(Debug, Deserialize)]
pub struct Doors {
    #[serde(rename = "frontLeft", deserialize_with = "int_to_bool")]
    pub front_left_opened: bool,

    #[serde(rename = "frontRight", deserialize_with = "int_to_bool")]
    pub front_right_opened: bool,

    #[serde(rename = "backLeft", deserialize_with = "int_to_bool")]
    pub back_left_opened: bool,

    #[serde(rename = "backRight", deserialize_with = "int_to_bool")]
    pub back_right_opened: bool,
}

fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i32 = de::Deserialize::deserialize(deserializer)?;

    Ok(value != 0)
}

#[derive(Debug, Deserialize)]
pub struct Battery {
    #[serde(rename = "batteryCharge")]
    pub is_charging: bool,

    #[serde(rename = "batteryStatus")]
    pub state_of_charge: u32,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    #[serde(rename = "coord")]
    pub coordinates: Coordinates,

    #[serde(rename = "accuracy")]
    pub precision_dilution: PrecisionDilution,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "lat")]
    pub latitude: f32,

    #[serde(rename = "lon")]
    pub longitude: f32,

    #[serde(rename = "alt")]
    pub altitude: f32,
}

/// [DOP] (Dilution of precision).
///
/// [DOP]: https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation)
#[derive(Debug, Deserialize)]
pub struct PrecisionDilution {
    #[serde(rename = "hdop")]
    pub horizontal: u32,

    #[serde(rename = "pdop")]
    pub position: u32,
}
