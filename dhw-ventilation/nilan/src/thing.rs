use crate::{reader, state::VentilationMode, state::VentilationState, writer};
use serde_json::{json, Map, Value};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock, Weak},
    thread, time,
};
use tokio_modbus::prelude::*;
use uuid::Uuid;
use webthing::{
    server, Action as ThingAction, BaseAction, BaseProperty, BaseThing, Thing, ThingsType,
    WebThingServer,
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
        "mode".to_owned(),
        json!("auto"),
        None,
        Some(
            json!({
                "@type": "ThermostatModeProperty",
                "title": "Ventilation mode",
                "type": "string",
                "enum": ["auto", "cool", "heat"],
                "description": "The ventilation mode",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "state".to_owned(),
        json!("running"),
        None,
        Some(
            json!({
                "@type": "ThermostatModeProperty",
                "title": "Ventilation state",
                "type": "string",
                "enum": ["paused", "running"],
                "description": "The ventilation state",
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
                "@type": "HumidityProperty",
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

    thing.add_available_action(
        "stop".to_owned(),
        json!({
            "title": "Stop",
            "description": "Stop the ventilation",
        })
        .as_object()
        .unwrap()
        .clone(),
    );
    thing.add_available_action(
        "start".to_owned(),
        json!({
            "title": "Start",
            "description": "Start the ventilation",
        })
        .as_object()
        .unwrap()
        .clone(),
    );

    Arc::new(RwLock::new(Box::new(thing)))
}

struct VentilationAction {
    inner: BaseAction,
    address: SocketAddr,
    new_state: VentilationState,
}

impl VentilationAction {
    fn new(
        input: Option<Map<String, Value>>,
        thing: Weak<RwLock<Box<dyn Thing>>>,
        action_name: String,
        address: SocketAddr,
        new_state: VentilationState,
    ) -> Self {
        Self {
            inner: BaseAction::new(Uuid::new_v4().to_string(), action_name, input, thing),
            address,
            new_state,
        }
    }
}

impl ThingAction for VentilationAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.inner.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.inner.get_id()
    }

    fn get_name(&self) -> String {
        self.inner.get_name()
    }

    fn get_href(&self) -> String {
        self.inner.get_href()
    }

    fn get_status(&self) -> String {
        self.inner.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.inner.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.inner.get_time_completed()
    }

    fn get_input(&self) -> Option<Map<String, Value>> {
        self.inner.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<dyn Thing>>>> {
        self.inner.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.inner.set_status(status)
    }

    fn start(&mut self) {
        self.inner.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();

        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let address = self.address.clone();
        let name = self.get_name();
        let id = self.get_id();
        let new_state = self.new_state.clone();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();

            println!("Updating ventilation state to `{:?}`", new_state);

            let mut context = sync::tcp::connect(address).unwrap();
            writer::set_ventilation_state(&mut context, new_state).unwrap();

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.inner.cancel()
    }

    fn finish(&mut self) {
        self.inner.finish()
    }
}

struct Generator {
    address: SocketAddr,
}

impl server::ActionGenerator for Generator {
    fn generate(
        &self,
        thing: Weak<RwLock<Box<dyn Thing>>>,
        name: String,
        input: Option<&Value>,
    ) -> Option<Box<dyn ThingAction>> {
        let input = input
            .and_then(|v| v.as_object())
            .and_then(|v| Some(v.clone()));

        match name.as_str() {
            "stop" => Some(Box::new(VentilationAction::new(
                input,
                thing,
                "stop".to_string(),
                self.address,
                VentilationState::Paused,
            ))),
            "start" => Some(Box::new(VentilationAction::new(
                input,
                thing,
                "start".to_string(),
                self.address,
                VentilationState::Running,
            ))),
            _ => None,
        }
    }
}

macro_rules! update_property(
    ($thing:expr, $property_name:expr, $value:expr) => {
        {
            let new_value = json!($value);

            let property_name = $property_name.to_string();
            let mut thing = $thing.write().expect("Cannot get a write lock on the thing.");
            let property = thing.find_property(&property_name).expect("Cannot find the property.");
            property.set_cached_value(new_value.clone()).expect("Cannot set the cached value");

            thing.property_notify(property_name, new_value);
        }
    };
);

pub fn run(address: SocketAddr, port: Option<u16>) {
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(2);

    let domestic_hot_water = make_domestic_hot_water();
    things.push(domestic_hot_water.clone());

    let ventilation = make_ventilation();
    things.push(ventilation.clone());

    thread::spawn(move || loop {
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
                "mode",
                match ventilation_state.mode {
                    VentilationMode::Auto => "auto",
                    VentilationMode::Cooling => "cool",
                    VentilationMode::Heating => "heat",
                }
            );
            update_property!(
                ventilation,
                "state",
                match ventilation_state.state {
                    VentilationState::Paused => "paused",
                    VentilationState::Running => "running",
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

        thread::sleep(time::Duration::from_secs(60));
    });

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Nilan".to_owned()),
        port,
        None,
        None,
        Box::new(Generator { address }),
        None,
        None,
    );
    server.create();
    server.start();
}
