use crate::events::Event;
use crate::state::{State, SunPeriod, UpdateState};
use reqwest::blocking::Client as HTTPClient;
use std::{net, thread, time::Duration};

pub fn run(blinds_url: &net::SocketAddr) {
    let mut new_events = Vec::new();
    let mut state = State::default();

    let blinds_url = format!("http://{}", blinds_url);

    let loupe = thread::spawn(move || loop {
        new_events.clear();

        state = state.update(&mut new_events);

        dbg!(&state);
        dbg!(&new_events);

        for new_event in &new_events {
            match new_event {
                Event::SunPeriodChange if state.sun.period == SunPeriod::Night => {
                    close_blinds(&blinds_url)
                }

                _ => {
                    // do nothing.
                }
            }
        }

        println!("Sleeping…");

        thread::sleep(Duration::from_secs(3));
        //
    });

    loupe
        .join()
        .expect("Something has failed in the event loop");
}

fn close_blinds(blinds_url: &str) {
    let client = HTTPClient::new();
    let _res = client
        .post(format!("{}/0/actions/close", blinds_url))
        .body("{\"open\": {}}")
        .send();
}
