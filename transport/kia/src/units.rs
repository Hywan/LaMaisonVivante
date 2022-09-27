use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use std::fmt;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Percent(u64);

impl fmt::Debug for Percent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}%", self.0)
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Coordinate(f32);

impl fmt::Debug for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}°", self.0)
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Meter(f32);

impl fmt::Debug for Meter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}m", self.0)
    }
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DistanceUnit {
    Kilometers = 1,
    Miles = 3,
}

pub struct Kilometer(f32);

impl fmt::Debug for Kilometer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}km", self.0)
    }
}

pub fn distance_to_km<'de, D>(deserializer: D) -> Result<Kilometer, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Pair {
        value: f32,
        unit: DistanceUnit,
    }

    let Pair { value, unit } = Deserialize::deserialize(deserializer)?;

    Ok(match unit {
        DistanceUnit::Kilometers => Kilometer(value),
        DistanceUnit::Miles => Kilometer(value * 1.609344),
    })
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TemperatureUnit {
    Celcius = 0,
    Farenheit = 1,
}

pub struct Celcius(f32);

impl fmt::Debug for Celcius {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}°C", self.0)
    }
}

pub fn temperature_to_celcius<'de, D>(deserializer: D) -> Result<Celcius, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Pair {
        value: String,
        unit: TemperatureUnit,
    }

    let Pair { value: temperature_index, unit } = Deserialize::deserialize(deserializer)?;

    let mut temperature_index = usize::from_str_radix(temperature_index.trim_end_matches('H'), 16).unwrap();
    let (start, end, step) = (14f32, 30f32, 0.5f32);
    let mut temperature = start;

    while temperature_index > 0 && temperature <= end {
        temperature += step;
        temperature_index -= 1;
    };

    Ok(match unit {
        TemperatureUnit::Celcius => Celcius(temperature),
        TemperatureUnit::Farenheit => Celcius((temperature - 32.) / 1.8),
    })
}
