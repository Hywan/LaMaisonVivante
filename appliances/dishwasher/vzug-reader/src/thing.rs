use crate::reader;
//use async_std::task;
use serde_json::{json, Value};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock, Weak},
    thread, time,
};
use tokio::runtime;
use webthing::{
    server, Action as ThingAction, BaseProperty, BaseThing, Thing, ThingsType, WebThingServer,
};

fn make_dishwasher() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:dishwasher".to_string(),
        "Dishwasher".to_string(),
        Some(vec![
            "MultiLevelSensor".to_string(),
            "EnergyMonitor".to_string(),
        ]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "average_power".to_string(),
        json!(false),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Average power consumption",
                "type": "number",
                "description": "The average power consumption",
                "unit": "watt",
                "readOnly": true,
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "average_water".to_string(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Average water consumption",
                "type": "number",
                "description": "The average water consumption",
                "minimum": 0,
                "maximum": 20,
                "unit": "liter",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "total_power".to_string(),
        json!(false),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Total power consumption",
                "type": "number",
                "description": "The total power consumption",
                "unit": "watt",
                "readOnly": true,
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "total_water".to_string(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "Total water consumption",
                "type": "number",
                "description": "The total water consumption",
                "minimum": 0,
                "maximum": 1000000,
                "unit": "liter",
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

    let dishwasher = make_dishwasher();
    things.push(dishwasher.clone());

    thread::spawn(move || loop {
        let state = runtime::Runtime::new()
            .unwrap()
            .block_on(reader::read(&address))
            .unwrap();

        // Average consumption.
        {
            let average_consumption_state = state.average_consumption;
            let dishwasher = dishwasher.clone();

            update_property!(dishwasher, "average_power", average_consumption_state.power);
            update_property!(dishwasher, "average_water", average_consumption_state.water);
        }

        // Total consumption.
        {
            let total_consumption_state = state.total_consumption;
            let dishwasher = dishwasher.clone();

            update_property!(dishwasher, "total_power", total_consumption_state.power);
            update_property!(dishwasher, "total_water", total_consumption_state.water);
        }

        // Program.
        {}

        thread::sleep(time::Duration::from_secs(2));
    });

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Lights".to_string()),
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
