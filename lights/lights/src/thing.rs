use crate::{
    command::{Action, Subject, ToString},
    writer,
};
use serde_json::{json, Value};
use std::{
    net::{TcpStream, ToSocketAddrs},
    sync::{Arc, RwLock, Weak},
};
use webthing::{
    property, server, Action as ThingAction, BaseProperty, BaseThing, Thing, ThingsType,
    WebThingServer,
};

struct PulseValueForwarder<A>
where
    A: ToSocketAddrs + Copy + Clone + Send + Sync,
{
    address: A,
    subject: Subject,
}

impl<A> property::ValueForwarder for PulseValueForwarder<A>
where
    A: ToSocketAddrs + Copy + Clone + Send + Sync,
{
    fn set_value(&mut self, value: Value) -> Result<Value, &'static str> {
        println!(
            "Sending a {:?} to {:?} (value `{}`)…",
            Action::Pulse,
            self.subject,
            value
        );

        let stream =
            TcpStream::connect(self.address).map_err(|_| "Failed to connect to the light")?;

        writer::send(&stream, self.subject, Action::Pulse)
            .map_err(|_| "Failed to send a pulse on a light")?;

        Ok(value)
    }
}

fn make_light<A>(address: A, subject: Subject) -> Arc<RwLock<Box<dyn Thing + 'static>>>
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
    let mut thing = BaseThing::new(
        format!("urn:dev:ops:light-{}", subject as u8),
        ToString::to_string(&subject),
        Some(vec!["Light".to_owned()]),
        None,
    );

    thing.add_property(Box::new(BaseProperty::new(
        "pulse".to_owned(),
        json!(false),
        Some(Box::new(PulseValueForwarder { address, subject })),
        Some(
            json!({
                "@type": "OnOffProperty",
                "title": "Pulse",
                "type": "boolean",
                "description": "Whether to turn the light on"
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

pub fn run<A>(address: A, port: Option<u16>)
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(1);

    things.push(make_light(address, Subject::LaundryRoom));
    things.push(make_light(address, Subject::Bathroom));
    things.push(make_light(address, Subject::LouiseBedroom));
    things.push(make_light(address, Subject::EliBedroom));
    things.push(make_light(address, Subject::Hall));
    things.push(make_light(address, Subject::LivingRoom));
    things.push(make_light(address, Subject::SittingRoom));
    things.push(make_light(address, Subject::DiningTable));
    things.push(make_light(address, Subject::KitchenIsland));
    things.push(make_light(address, Subject::Kitchen));
    things.push(make_light(address, Subject::ParentBed));
    things.push(make_light(address, Subject::ParentBathroom));
    things.push(make_light(address, Subject::ParentBedroom));
    things.push(make_light(address, Subject::GreenHouse));

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
