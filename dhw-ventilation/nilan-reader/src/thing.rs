use crate::{reader, state::VentilationState};
use serde_json::{json, Value};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock, Weak},
    thread, time,
};
use tokio_modbus::prelude::*;
use webthing::{
    server, Action as ThingAction, BaseProperty, BaseThing, Thing, ThingsType, WebThingServer,
};

fn make_domestic_hot_water() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:domestic-hot-water".to_owned(),
        "Domestic Hot Water".to_owned(),
        Some(vec!["TemperatureSensor".to_owned()]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "top_of_the_tank".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Top of the tank",
                "type": "integer",
                "description": "The temperature of the top of the tank",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "bottom_of_the_tank".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Bottom of the tank",
                "type": "integer",
                "description": "The temperature of the bottom of the tank",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "wanted".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TargetTemperatureProperty",
                "title": "Targeted temperature",
                "type": "integer",
                "description": "The targeted temperature of the tank",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    Arc::new(RwLock::new(Box::new(thing)))
}

fn make_ventilation() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:ventilation".to_owned(),
        "Ventilation".to_owned(),
        Some(vec!["Thermostat".to_owned(), "MultiLevelSensor".to_owned()]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "state".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "ThermostatModeProperty",
                "title": "Ventilation state",
                "type": "string",
                "enum": ["auto", "cool", "heat"],
                "description": "The ventilation state/mode",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "wanted_air_inside".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TargetTemperatureProperty",
                "title": "Wanted air inside",
                "type": "integer",
                "description": "The wanted inside air temperature",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "supplied_air_after_ground_coupled_heat_exchanger".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Supplied air after ground-coupled heat exchanger",
                "type": "integer",
                "description": "The supplied air temperature after the ground-coupled heat exchanger",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "supplied_air_after_heat_recovery_exchanger".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Supplied air after heat recovery exchanger",
                "type": "integer",
                "description": "The supplied air temperature after the heat recovery exchanger",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "extracted_air".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Extracted air",
                "type": "integer",
                "description": "The extracted air temperature",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "discharged_air".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Discharged air",
                "type": "integer",
                "description": "The discharged air temperature",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "inside_air_humidity".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Inside air humidity",
                "type": "integer",
                "description": "The inside air humidity",
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
        "inside_co2_level".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Inside CO2 level",
                "type": "integer",
                "description": "The inside CO2 level",
                "minimum": 0,
                "maximum": 1000,
                "unit": "ppm",
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

pub fn run(address: SocketAddr, port: Option<u16>) {
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(1);

    let domestic_hot_water = make_domestic_hot_water();
    things.push(domestic_hot_water.clone());

    let ventilation = make_ventilation();
    things.push(ventilation.clone());

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(60));

        let mut context = match sync::tcp::connect(address) {
            Ok(e) => e,
            _ => return, // silently fail
        };

        // Reading the current state.
        let state = reader::read(&mut context).unwrap_or_else(|_| Default::default());

        // Domestic Hot Water
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

        // Ventilation
        {
            let ventilation_state = state.ventilation;
            let ventilation = ventilation.clone();

            update_property!(
                ventilation,
                "state",
                match ventilation_state.state {
                    VentilationState::Auto => "auto",
                    VentilationState::Cooling => "cool",
                    VentilationState::Heating => "heat",
                }
            );
            update_property!(
                ventilation,
                "wanted_air_inside",
                ventilation_state.temperatures.wanted_inside_air
            );
            update_property!(
                ventilation,
                "supplied_air_after_ground_coupled_heat_exchanger",
                ventilation_state
                    .temperatures
                    .supplied_air_after_ground_coupled_heat_exchanger
            );
            update_property!(
                ventilation,
                "supplied_air_after_heat_recovery_exchanger",
                ventilation_state
                    .temperatures
                    .supplied_air_after_heat_recovery_exchanger
            );
            update_property!(
                ventilation,
                "extracted_air",
                ventilation_state.temperatures.extracted_air
            );
            update_property!(
                ventilation,
                "discharged_air",
                ventilation_state.temperatures.discharged_air
            );
            update_property!(
                ventilation,
                "inside_air_humidity",
                ventilation_state.inside_air_humidity
            );
            update_property!(
                ventilation,
                "inside_co2_level",
                ventilation_state.inside_co2_level
            );
        }
    });

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Lights".to_owned()),
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
