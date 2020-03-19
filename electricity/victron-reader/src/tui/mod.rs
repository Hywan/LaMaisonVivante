mod drawer;

use crate::{reader, state::*};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use drawer::Drawer;
use std::{
    io::{self, Stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use tokio_modbus::prelude::*;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    terminal::Frame,
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

pub(crate) struct Application<'a> {
    states: Vec<State>,
    context: &'a mut sync::Context,
}

impl<'a> Application<'a> {
    fn new(context: &'a mut sync::Context) -> Self {
        Self {
            states: vec![],
            context: context,
        }
    }

    fn tick(&mut self) -> io::Result<()> {
        let new_state = reader::read(self.context)?;

        if self.states.len() == 50 {
            self.states.remove(0);
        }

        self.states.push(new_state);

        Ok(())
    }

    fn last_state(&self) -> &State {
        assert!(self.states.len() >= 1, "There must always be one state.");

        &self.states[self.states.len() - 1]
    }
}

fn draw(application: &Application, frame: &mut Frame<CrosstermBackend<Stdout>>) -> () {
    let (battery_panel, pv_inverter_panel, vebus_panel, house_panel) = {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Min(13),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(frame.size());

        (rects[0], rects[1], rects[2], rects[3])
    };

    Battery::draw(application, frame, battery_panel);
    PvInverter::draw(application, frame, pv_inverter_panel);
    Vebus::draw(application, frame, vebus_panel);
    House::draw(application, frame, house_panel);
}

pub fn run(mut context: &mut sync::Context) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let CEvent::Key(key) = event::read().unwrap() {
                tx.send(Event::Input(key)).unwrap();
            }
        }

        tx.send(Event::Tick).unwrap();
    });

    terminal.clear()?;

    let mut application = Application::new(&mut context);
    application.tick()?;

    loop {
        terminal.draw(|mut frame| {
            draw(&application, &mut frame);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;

                    break;
                }
                _ => {}
            },
            Event::Tick => application.tick()?,
        };
    }

    Ok(())
}
