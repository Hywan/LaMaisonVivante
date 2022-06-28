use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Thing {
    pub id: String,
    pub base: String,
    pub title: String,
    ///#[serde(rename(deserialize = "@type"))]
    ///pub capabilities: Vec<Capability>,
    pub properties: HashMap<String, Property>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    pub title: String,
    #[serde(rename(deserialize = "@type"))]
    pub r#type: PropertyType,
    //pub unit: String,
    //#[serde(rename(deserialize = "readOnly"))]
    //pub read_only: bool,
    pub value: Option<PropertyValue>,
}

pub type PropertyValue = Value;

// https://webthings.io/schemas/#capabilities
#[derive(Debug, Deserialize)]
pub enum Capability {
    Alarm,
    AirQualitySensor,
    BarometricPressureSensor,
    BinarySensor,
    Camera,
    ColorControl,
    ColorSensor,
    DoorSensor,
    EnergyMonitor,
    HumiditySensor,
    LeakSensor,
    Light,
    Lock,
    MotionSensor,
    MultiLevelSensor,
    MultiLevelSwitch,
    OnOffSwitch,
    PushButton,
    SmartPlug,
    SmokeSensor,
    TemperatureSensor,
    Thermostat,
    VideoCamera,
}

// https://webthings.io/schemas/#properties
#[derive(Debug, Deserialize)]
pub enum PropertyType {
    #[serde(rename(deserialize = "AlarmProperty"))]
    Alarm,
    #[serde(rename(deserialize = "BarometricPressureProperty"))]
    BarometricPressure,
    #[serde(rename(deserialize = "BooleanProperty"))]
    Boolean,
    #[serde(rename(deserialize = "BrightnessProperty"))]
    Brightness,
    #[serde(rename(deserialize = "ColorModeProperty"))]
    ColorMode,
    #[serde(rename(deserialize = "ColorProperty"))]
    Color,
    #[serde(rename(deserialize = "ColorTemperatureProperty"))]
    ColorTemperature,
    #[serde(rename(deserialize = "ConcentrationProperty"))]
    Concentration,
    #[serde(rename(deserialize = "CurrentProperty"))]
    Current,
    #[serde(rename(deserialize = "DensityProperty"))]
    Density,
    #[serde(rename(deserialize = "FrequencyProperty"))]
    Frequency,
    #[serde(rename(deserialize = "HeatingCoolingProperty"))]
    HeatingCooling,
    #[serde(rename(deserialize = "HumidityProperty"))]
    Humidity,
    #[serde(rename(deserialize = "ImageProperty"))]
    Image,
    #[serde(rename(deserialize = "InstantaneousPowerFactorProperty"))]
    InstantaneousPowerFactor,
    #[serde(rename(deserialize = "InstantaneousPowerProperty"))]
    InstantaneousPower,
    #[serde(rename(deserialize = "LeakProperty"))]
    Leak,
    #[serde(rename(deserialize = "LevelProperty"))]
    Level,
    #[serde(rename(deserialize = "LockedProperty"))]
    Locked,
    #[serde(rename(deserialize = "MotionProperty"))]
    Motion,
    #[serde(rename(deserialize = "OnOffProperty"))]
    OnOff,
    #[serde(rename(deserialize = "OpenProperty"))]
    Open,
    #[serde(rename(deserialize = "PushedProperty"))]
    Pushed,
    #[serde(rename(deserialize = "SmokeProperty"))]
    Smoke,
    #[serde(rename(deserialize = "TargetTemperatureProperty"))]
    TargetTemperature,
    #[serde(rename(deserialize = "TemperatureProperty"))]
    Temperature,
    #[serde(rename(deserialize = "ThermostatModeProperty"))]
    ThermostatMode,
    #[serde(rename(deserialize = "VideoProperty"))]
    Video,
    #[serde(rename(deserialize = "VoltageProperty"))]
    Voltage,

    // Non-standard.
    #[serde(rename(deserialize = "RecurrenceProperty"))]
    Recurrence,
}
