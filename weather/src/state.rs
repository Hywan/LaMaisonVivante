use chrono::{prelude::*, serde::ts_seconds};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub const HOME_LATITUDE: f32 = 46.78657339107215;
pub const HOME_LONGITUDE: f32 = 6.806581635522576;
pub const HOME_LANGUAGE: &str = "fr";

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Precipitation {
    #[serde(rename(deserialize = "1h"))]
    pub one_hour: f32,
    #[serde(rename(deserialize = "3h"))]
    pub three_hour: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub clouds: i32,
    #[serde(rename(deserialize = "dt"), with = "ts_seconds")]
    pub datetime: DateTime<Utc>,
    #[serde(rename(deserialize = "temp"))]
    pub temperature: f32,
    #[serde(rename(deserialize = "feels_like"))]
    pub apparent_temperature: f32,
    pub humidity: i32,
    pub dew_point: f32,
    pub pressure: i32,
    pub sunrise: Option<i32>,
    pub sunset: Option<i32>,
    #[serde(rename(deserialize = "uvi"))]
    pub uv_index: f32,
    pub visibility: Option<i32>,
    #[serde(rename(deserialize = "wind_deg"))]
    pub wind_degree: i32,
    pub wind_speed: f32,
    pub wind_gust: Option<f32>,
    pub snow: Option<Precipitation>,
    pub rain: Option<Precipitation>,
    #[serde(rename(deserialize = "weather"))]
    pub conditions: Vec<WeatherCondition>,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            clouds: Default::default(),
            datetime: Utc::now(),
            temperature: Default::default(),
            apparent_temperature: Default::default(),
            humidity: Default::default(),
            dew_point: Default::default(),
            pressure: Default::default(),
            sunrise: Default::default(),
            sunset: Default::default(),
            uv_index: Default::default(),
            visibility: Default::default(),
            wind_degree: Default::default(),
            wind_speed: Default::default(),
            wind_gust: Default::default(),
            snow: Default::default(),
            rain: Default::default(),
            conditions: Default::default(),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum WeatherConditionId {
    ThunderstormWithLightRain = 200,
    ThunderstormWithRain = 201,
    ThunderstormWithHeavyRain = 202,
    LightThunderstorm = 210,
    Thunderstorm = 211,
    HeavyThunderstorm = 212,
    RaggedThunderstorm = 221,
    ThunderstormWithLightDrizzle = 230,
    ThunderstormWithDrizzle = 231,
    ThunderstormWithHeavyDrizzle = 232,

    LightIntensityDrizzle = 300,
    Drizzle = 301,
    HeavyIntensityDrizzle = 302,
    LightIntensityDrizzleRain = 310,
    DrizzleRain = 311,
    HeavyIntensityDrizzleRain = 312,
    ShowerRainAndDrizzle = 313,
    HeavyShowerRainAndDrizzle = 314,
    ShowerDrizzle = 321,

    LightRain = 500,
    ModerateRain = 501,
    HeavyIntensityRain = 502,
    VeryHeavyRain = 503,
    ExtremeRain = 504,
    FreezingRain = 511,
    LightIntensityShowerRain = 520,
    ShowerRain = 521,
    HeavyIntensityShowerRain = 522,
    RaggedShowerRain = 531,

    LightSnow = 600,
    Snow = 601,
    HeavySnow = 602,
    Sleet = 611,
    LightShowerSleet = 612,
    ShowerSleet = 613,
    LightRainAndSnow = 615,
    RainAndSnow = 616,
    LightShowerSnow = 620,
    ShowerSnow = 621,
    HeavyShowerSnow = 622,

    Mist = 701,
    Smoke = 711,
    Haze = 721,
    SandOrDustWhirls = 731,
    Fog = 741,
    Sand = 751,
    Dust = 761,
    VolcanicAsh = 762,
    Squalls = 771,
    Tornado = 781,

    ClearSky = 800,

    FewClouds = 801,
    ScatteredClouds = 802,
    BrokenClouds = 803,
    OvercastClouds = 804,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub description: String,
    pub id: WeatherConditionId,
}

impl Default for WeatherCondition {
    fn default() -> Self {
        Self {
            description: "".to_string(),
            id: WeatherConditionId::ClearSky,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub description: String,
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub end: DateTime<Utc>,
    #[serde(rename(deserialize = "sender_name"))]
    pub sender: String,
    pub event: String,
    pub tags: Vec<String>,
}

impl Default for Alert {
    fn default() -> Self {
        Self {
            description: Default::default(),
            start: Utc::now(),
            end: Utc::now(),
            sender: Default::default(),
            event: Default::default(),
            tags: Default::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub alerts: Option<Vec<Alert>>,
    #[serde(rename(deserialize = "current"))]
    pub current_weather: Weather,
    #[serde(rename(deserialize = "hourly"))]
    pub hourly_weather: Vec<Weather>,
}
