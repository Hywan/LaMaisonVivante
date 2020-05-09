use crate::{
    command::{Action, Subject, ToString},
    writer,
};
use serde_json::{json, Map, Value};
use std::{
    marker::{Send, Sync},
    net::{TcpStream, ToSocketAddrs},
    str::FromStr,
    sync::{Arc, RwLock, Weak},
    thread,
};
use uuid::Uuid;
use webthing::{
    server, Action as ThingAction, BaseAction, BaseThing, Thing, ThingsType, WebThingServer,
};

const THING_ID_PREFIX: &'static str = "urn:dev:ops:blind-";

fn make_blind(subject: Subject) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        format!("{}{}", THING_ID_PREFIX, subject as u8),
        ToString::to_string(&subject),
        Some(vec!["PushButton".to_owned()]),
        None,
    );

    thing.add_available_action(
        "open".to_owned(),
        json!({
            "title": "Open",
            "description": "Open the blind",
        })
        .as_object()
        .unwrap()
        .clone(),
    );
    thing.add_available_action(
        "close".to_owned(),
        json!({
            "title": "Close",
            "description": "Close the blind",
        })
        .as_object()
        .unwrap()
        .clone(),
    );
    thing.add_available_action(
        "stop".to_owned(),
        json!({
            "title": "Stop",
            "description": "Stop the blind",
        })
        .as_object()
        .unwrap()
        .clone(),
    );

    Arc::new(RwLock::new(Box::new(thing)))
}

struct BlindAction<A>
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
    inner: BaseAction,
    address: A,
    action: Action,
}

impl<A> BlindAction<A>
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
    fn new(
        input: Option<Map<String, Value>>,
        thing: Weak<RwLock<Box<dyn Thing>>>,
        action_name: String,
        address: A,
        action: Action,
    ) -> Self {
        Self {
            inner: BaseAction::new(Uuid::new_v4().to_string(), action_name, input, thing),
            address,
            action,
        }
    }
}

impl<A> ThingAction for BlindAction<A>
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
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
        let action = self.action.clone();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();
            let thing_id = thing.get_id();

            let subject =
                Subject::from(u8::from_str(thing_id.split_at(THING_ID_PREFIX.len()).1).unwrap());

            println!("Sending a {:?} to {:?}…", &action, &subject);

            let stream = TcpStream::connect(address).unwrap();

            writer::send(&stream, subject, action).unwrap();

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

struct Generator<A>
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
    address: A,
}

impl<A> server::ActionGenerator for Generator<A>
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
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
            "open" => Some(Box::new(BlindAction::new(
                input,
                thing,
                "open".to_string(),
                self.address,
                Action::Opening,
            ))),
            "close" => Some(Box::new(BlindAction::new(
                input,
                thing,
                "close".to_string(),
                self.address,
                Action::Closing,
            ))),
            "stop" => Some(Box::new(BlindAction::new(
                input,
                thing,
                "stop".to_string(),
                self.address,
                Action::Unmoving,
            ))),
            _ => None,
        }
    }
}

pub fn run<A>(address: A, port: Option<u16>)
where
    A: 'static + ToSocketAddrs + Copy + Clone + Send + Sync,
{
    let mut things: Vec<Arc<RwLock<Box<dyn Thing + 'static>>>> = Vec::with_capacity(1);

    things.push(make_blind(Subject::Kitchen));
    things.push(make_blind(Subject::LivingRoom));
    things.push(make_blind(Subject::ParentBedroom));
    things.push(make_blind(Subject::EliBedroom));
    things.push(make_blind(Subject::LouiseBedroom));
    things.push(make_blind(Subject::Bathroom));

    println!(
        "Starting the Things server (port {})…",
        port.map(|p| p.to_string())
            .unwrap_or_else(|| "[default]".to_string())
    );

    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Blinds".to_owned()),
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
