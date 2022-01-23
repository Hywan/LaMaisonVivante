use crate::state::{State, UpdateState};
use std::thread;
use std::time::Duration;

pub fn run() {
    let mut new_events = Vec::new();
    let mut current_state = State::default();

    let loupe = thread::spawn(move || loop {
        new_events.clear();

        let next_state = current_state.update(&mut new_events);
        dbg!(&next_state);
        dbg!(&new_events);

        current_state = next_state;

        println!("Sleeping…");

        thread::sleep(Duration::from_secs(3));
        //
    });

    loupe
        .join()
        .expect("Something has failed in the event loop");
}
