use crate::reader;
use serde_json::{json, Value};
use std::{
    sync::{Arc, RwLock, Weak},
    thread, time,
};
use tokio_modbus::prelude::*;
use webthing::{
    server, Action as ThingAction, BaseProperty, BaseThing, Thing, ThingsType, WebThingServer,
};

fn make_battery() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:ops:battery".to_owned(),
        "Batteries".to_owned(),
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
                "description": "The battery state of charge",
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
        "ongoing_power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Ongoing Power",
                "type": "integer",
                "description": "The battery ongoing power",
                "unit": "watt",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "voltage".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "VoltageProperty",
                "title": "Voltage",
                "type": "integer",
                "description": "The battery voltage",
                "unit": "volt",
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
                "type": "integer",
                "description": "The battery temperature",
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

fn make_pv_inverter(n: u8) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        format!("urn:dev:ops:pv-inverter-{}", n),
        format!("PV Inverter #{}", n),
        Some(vec!["EnergyMonitor".to_owned()]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Power",
                "type": "integer",
                "description": "The PV inverter power",
                "unit": "watt",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "voltage".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "VoltageProperty",
                "title": "Voltage",
                "type": "integer",
                "description": "The PV inverter voltage",
                "unit": "volt",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "current".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "CurrentProperty",
                "title": "Current",
                "type": "integer",
                "description": "The PV inverter current",
                "unit": "ampere",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "frequency".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "FrequencyProperty",
                "title": "Frequency",
                "type": "number",
                "description": "The PV inverter frequency",
                "unit": "hertz",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));

    Arc::new(RwLock::new(Box::new(thing)))
}

fn make_house() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        format!("urn:dev:ops:house-power"),
        "House Power".to_string(),
        Some(vec!["EnergyMonitor".to_owned()]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Power",
                "type": "integer",
                "description": "The house power consumption",
                "unit": "watt",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "l1_power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Power",
                "type": "integer",
                "description": "The house power consumption (phase 1)",
                "unit": "watt",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "l2_power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Power",
                "type": "integer",
                "description": "The house power consumption (phase 2)",
                "unit": "watt",
                "readOnly": true
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    )));
    thing.add_property(Box::new(BaseProperty::new(
        "l3_power".to_owned(),
        json!(0),
        None,
        Some(
            json!({
                "@type": "InstantaneousPowerProperty",
                "title": "Power",
                "type": "integer",
                "description": "The house power consumption (phase 3)",
                "unit": "watt",
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

pub fn run(mut context: sync::Context, port: Option<u16>) {
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(1);

    let battery = make_battery();
    things.push(battery.clone());

    let pv_inverter_1 = make_pv_inverter(1);
    things.push(pv_inverter_1.clone());

    let pv_inverter_2 = make_pv_inverter(2);
    things.push(pv_inverter_2.clone());

    let pv_inverter_3 = make_pv_inverter(3);
    things.push(pv_inverter_3.clone());

    let pv_inverter_all = make_pv_inverter(0);
    things.push(pv_inverter_all.clone());

    let house = make_house();
    things.push(house.clone());

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(2));

        // Reading the current state.
        let state = reader::read(&mut context).unwrap();
        let vebus_state = state.vebus.unwrap_or_else(|| Default::default());

        // Battery.
        {
            let battery_state = state.battery.unwrap_or_else(|| Default::default());
            let battery = battery.clone();

            update_property!(battery, "ongoing_power", battery_state.ongoing_power);
            update_property!(battery, "voltage", battery_state.voltage);
            update_property!(battery, "temperature", battery_state.temperature);
            update_property!(battery, "state_of_charge", battery_state.state_of_charge);
        }

        let pv_inverter_state = state.pv_inverter.unwrap_or_else(|| Default::default());

        // PV Inverter #1
        {
            let pv_inverter_state = pv_inverter_state.l1;
            let pv_inverter = pv_inverter_1.clone();

            update_property!(pv_inverter, "power", pv_inverter_state.power);
            update_property!(pv_inverter, "voltage", pv_inverter_state.voltage);
            update_property!(pv_inverter, "current", pv_inverter_state.current);
            update_property!(pv_inverter, "frequency", vebus_state.frequency);
        }

        // PV Inverter #2
        {
            let pv_inverter_state = pv_inverter_state.l2;
            let pv_inverter = pv_inverter_2.clone();

            update_property!(pv_inverter, "power", pv_inverter_state.power);
            update_property!(pv_inverter, "voltage", pv_inverter_state.voltage);
            update_property!(pv_inverter, "current", pv_inverter_state.current);
            update_property!(pv_inverter, "frequency", vebus_state.frequency);
        }

        // PV Inverter #3
        {
            let pv_inverter_state = pv_inverter_state.l3;
            let pv_inverter = pv_inverter_3.clone();

            update_property!(pv_inverter, "power", pv_inverter_state.power);
            update_property!(pv_inverter, "voltage", pv_inverter_state.voltage);
            update_property!(pv_inverter, "current", pv_inverter_state.current);
            update_property!(pv_inverter, "frequency", vebus_state.frequency);
        }

        // PV Inverter (all)
        {
            let pv_inverter_state = pv_inverter_state.clone();
            let pv_inverter = pv_inverter_all.clone();

            update_property!(
                pv_inverter,
                "power",
                pv_inverter_state.l1.power
                    + pv_inverter_state.l2.power
                    + pv_inverter_state.l3.power
            );
            update_property!(pv_inverter, "voltage", pv_inverter_state.l1.voltage);
            update_property!(pv_inverter, "current", pv_inverter_state.l1.current);
            update_property!(pv_inverter, "frequency", vebus_state.frequency);
        }

        // House
        {
            let house_state = state.house.unwrap_or_else(|| Default::default());
            let house = house.clone();

            update_property!(
                house,
                "power",
                house_state.l1 + house_state.l2 + house_state.l3
            );
            update_property!(house, "l1_power", house_state.l1);
            update_property!(house, "l2_power", house_state.l2);
            update_property!(house, "l3_power", house_state.l3);
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
