use crate::{
    reader,
    state::{SocketAvailability, SocketStatus},
};
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

fn make_charging_station() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:car-charging-station".to_owned(),
        "Car Charging Station".to_owned(),
        Some(vec![
            "EnergyMonitor".to_owned(),
            "TemperatureSensor".to_owned(),
        ]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "station_temperature".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "TemperatureProperty",
                "title": "Station inside temperature",
                "type": "integer",
                "description": "The temperature from inside the charging station",
                "unit": "celsius",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "max_current".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "CurrentProperty",
                "title": "Max current for the station",
                "type": "integer",
                "description": "The maximum current the station can deliver",
                "unit": "ampere",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "socket_availability".to_owned(),
        json!(false),
        None,
        Some(
            json!({
                "@type": "BooleanProperty",
                "title": "Socket availability",
                "type": "boolean",
                "description": "Whether the socket available",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "socket_charging".to_owned(),
        json!(false),
        None,
        Some(
            json!({
                "@type": "BooleanProperty",
                "title": "Socket charging",
                "type": "boolean",
                "description": "Whether the socket is charging the car",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "socket_number_of_phases".to_owned(),
        json!(3),
        None,
        Some(
            json!({
                "@type": "BooleanProperty",
                "title": "Socket charging",
                "type": "integer",
                "description": "Number of phases the socket is using",
                "readOnly": true,
                "minimum": 1,
                "maximum": 3
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "socket_power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Socket total power",
                "type": "integer",
                "description": "Total power given by the socket",
                "unit": "watt",
                "readOnly": true,
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "socket_frequency".to_owned(),
        json!(50),
        None,
        Some(
            json!({
                "@type": "FrequencyProperty",
                "title": "Socket frequency",
                "type": "hertz",
                "description": "Frequency of the socket",
                "readOnly": true,
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "socket_current".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "CurrentProperty",
                "title": "Socket current",
                "type": "integer",
                "description": "Actual applied max current of the socket",
                "unit": "ampere",
                "readOnly": true,
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
    ($thing:expr, $property_name:expr, $value:expr $(,)*) => {
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
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(1);

    let charging_station = make_charging_station();
    things.push(charging_station.clone());

    thread::spawn(move || loop {
        let mut context = match sync::tcp::connect(address) {
            Ok(e) => e,
            _ => return, // silently fail
        };

        // Reading the current state.
        let state = reader::read(&mut context).unwrap_or_else(|_| Default::default());

        // Charging station status.
        {
            let status_state = state.station_status;
            let socket_state = state.socket;
            let charging_station = charging_station.clone();

            update_property!(
                charging_station,
                "station_temperature",
                status_state.temperature,
            );
            update_property!(charging_station, "max_current", status_state.max_current);
            update_property!(
                charging_station,
                "socket_availability",
                matches!(socket_state.availability, SocketAvailability::Operative),
            );
            update_property!(
                charging_station,
                "socket_charging",
                matches!(socket_state.status, SocketStatus::Charging),
            );
            update_property!(
                charging_station,
                "socket_number_of_phases",
                socket_state.number_of_phases.as_u8(),
            );
            update_property!(
                charging_station,
                "socket_power",
                socket_state.power.0.round()
            );
            update_property!(charging_station, "socket_frequency", socket_state.frequency);
            update_property!(
                charging_station,
                "socket_current",
                socket_state.session.actual_applied_max_current
            );
        }

        thread::sleep(time::Duration::from_secs(10));
    });

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Alfen".to_owned()),
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
