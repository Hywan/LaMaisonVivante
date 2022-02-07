use crate::{reader, state};
use serde_json::{json, Value};
use std::{
    sync::{Arc, RwLock, Weak},
    thread, time,
};
use webthing::{
    server, Action as ThingAction, BaseProperty, BaseThing, Thing, ThingsType, WebThingServer,
};

fn make_current_weather() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:current_weather".to_owned(),
        "Current Weather".to_owned(),
        Some(vec![
            "MultiLevelSensor".to_owned(),
            "HumiditySensor".to_owned(),
            "TemperatureSensor".to_owned(),
            "BarometricPressureSensor".to_owned(),
        ]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "clouds".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Clouds",
                "type": "number",
                "description": "Cloudiness of the sky",
                "minimum": 0,
                "maximum": 100,
                "unit": "percent",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "temperature".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Temperature",
                "type": "number",
                "description": "The measured temperature",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "apparent_temperature".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "The apparent temperature",
                "type": "number",
                "description": "The apparent temperature",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "humidity".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "HumidityProperty",
                "title": "Humidity",
                "type": "integer",
                "description": "The measured humidity",
                "unit": "percent",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "dew_point".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Dew point",
                "type": "number",
                "description": "The dew point",
                "unit": "percent",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "pressure".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "BarometricPressureProperty",
                "title": "Pressure",
                "type": "integer",
                "description": "The barometric pressure",
                "unit": "hectopascal",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "sunrise".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "DateProperty", // not standard
                "title": "Sunrise",
                "type": "integer",
                "description": "The sunrise timestamp",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "sunset".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "DateProperty", // not standard
                "title": "Sunset",
                "type": "integer",
                "description": "The sunset timestamp",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "uv_index".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "UV index",
                "type": "number",
                "description": "The UV index",
                "minimum": 0,
                "maximum": 12,
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "visibility".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Visibility distance",
                "type": "integer",
                "description": "The visibility range",
                "minimum": 0,
                "unit": "meter",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "wind_degree".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Wind orientation",
                "type": "integer",
                "description": "The wind orientation",
                "minimum": 0,
                "maximum": 360,
                "unit": "degree",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "wind_speed".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Wind orientation",
                "type": "number",
                "description": "The wind speed",
                "minimum": 0,
                "unit": "km/sec",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "condition".to_owned(),
        json!(800),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Condition ID",
                "type": "integer",
                "description": "The weather condition ID",
                "minimum": 200,
                "maximum": 900,
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    Arc::new(RwLock::new(Box::new(thing)))
}

struct Generator;

impl server::ActionGenerator for Generator {
    fn generate(
        &self,
        _thing: Weak<RwLock<Box<dyn Thing>>>,
        _name: String,
        _input: Option<&Value>,
    ) -> Option<Box<dyn ThingAction>> {
        None
    }
}

macro_rules! update_property(
    ($thing:expr, $property_name:expr, $value:expr) => {
        {
            let new_value = json!($value);

            let property_name = $property_name.to_string();
            let mut thing = $thing.write().unwrap();
            let property = thing.find_property(&property_name).unwrap();
            property.set_cached_value(new_value.clone()).unwrap();

            thing.property_notify(property_name, new_value);
        }
    };
);

pub fn run(openweathermap_api_key: &str, port: Option<u16>) {
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(2);

    let current_weather = make_current_weather();
    things.push(current_weather.clone());

    let openweathermap_api_key = openweathermap_api_key.to_string();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(60));

        // Reading the current state.
        let state = reader::read(&openweathermap_api_key).unwrap_or_else(|_| Default::default());

        // Current weather.
        /*
        {
            let domestic_hot_water_state = state.domestic_hot_water;
            let storage_temperatures = domestic_hot_water_state.storage_temperatures;
            let domestic_hot_water = domestic_hot_water.clone();

            update_property!(
                domestic_hot_water,
                "top_of_the_tank",
                storage_temperatures.top_of_the_tank
            );
            update_property!(
                domestic_hot_water,
                "bottom_of_the_tank",
                storage_temperatures.bottom_of_the_tank
            );
            update_property!(domestic_hot_water, "wanted", storage_temperatures.wanted);
        }
        */
    });

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Weather".to_owned()),
        port,
        None,
        None,
        Box::new(Generator),
        None,
        None,
    );
    server.create();
    server.start();
}
