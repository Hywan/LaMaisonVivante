use crate::actions;
use crate::events::Event;
use crate::state::{State, SunPeriod, UpdateState};
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
                    actions::close_blinds(&blinds_url).unwrap()
                }

                _ => {
                    // do nothing.
                }
            }
        }

        println!("Sleeping…");

        thread::sleep(Duration::from_secs(30));
    });

    loupe
        .join()
        .expect("Something has failed in the event loop");
}
