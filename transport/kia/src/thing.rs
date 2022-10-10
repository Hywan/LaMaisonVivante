use crate::{
    auth::Authentification,
    brand::{Brand, Region},
    garage::Garage,
};
use serde_json::{json, Value};
use std::{
    sync::{Arc, RwLock, Weak},
    thread, time,
};
use webthing::{
    server, Action as ThingAction, BaseProperty, BaseThing, Thing, ThingsType, WebThingServer,
};

fn make_vehicle() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:vehicle".to_owned(),
        "Vehicle".to_owned(),
        Some(vec!["MultiLevelSensor".to_owned()]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "state_of_charge".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "LevelProperty",
                "title": "State of Charge",
                "type": "number",
                "description": "The vehicle state of charge",
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
        "description".to_owned(),
        json!({}),
        None,
        Some(
            json!({
                "@type": "HeterogeneousCollectionProperty", // not standard
                "title": "Description",
                "type": "object",
                "description": "Description of the vehicle",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    thing.add_property(Box::new(BaseProperty::new(
        "state".to_owned(),
        json!({}),
        None,
        Some(
            json!({
                "@type": "HeterogeneousCollectionProperty", // not standard
                "title": "State",
                "type": "object",
                "description": "State of the vehicle",
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

pub fn run(auth: Authentification, port: Option<u16>) {
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(1);

    let vehicle = make_vehicle();
    things.push(vehicle.clone());

    tokio::spawn(async move {
        loop {
            let garage = Garage::new(Region::Europe, Brand::Kia, &auth)
                .await
                .unwrap();
            let vehicles = garage.vehicles().await.unwrap();
            let vehicle0 = vehicles.iter().nth(0).unwrap();

            // Vehicle status.
            {
                let state = vehicle0.state().await.unwrap();
                let vehicle = vehicle.clone();

                update_property!(
                    vehicle,
                    "state_of_charge",
                    state.status.battery.state_of_charge,
                );
                update_property!(vehicle, "description", vehicle0);
                update_property!(vehicle, "state", state);
            }

            tokio::time::sleep(time::Duration::from_secs(60)).await;
        }
    });

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Kia".to_owned()),
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
