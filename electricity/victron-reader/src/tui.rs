use crate::reader;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use tokio_modbus::prelude::*;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
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
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let CEvent::Key(key) = event::read().unwrap() {
                tx.send(Event::Input(key)).unwrap();
            }
        }

        tx.send(Event::Tick).unwrap();
    });

    terminal.clear()?;

    let mut state = reader::read(&mut context)?;

    loop {
        terminal.draw(|mut frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let block_battery = Block::default().title("Battery").borders(Borders::ALL);
            let block_pv_inverter = Block::default().title("PV Inverter").borders(Borders::ALL);
            let block_vebus = Block::default().title("Vebus").borders(Borders::ALL);
            let block_house = Block::default().title("House").borders(Borders::ALL);

            Paragraph::new(
                [Text::raw(match &state.battery {
                    Some(battery) => battery.to_string(),
                    None => "n/a".to_string(),
                })]
                .iter(),
            )
            .block(block_battery)
            .wrap(true)
            .render(&mut frame, chunks[0]);

            Paragraph::new(
                [Text::raw(match &state.pv_inverter {
                    Some(pv_inverter) => pv_inverter.to_string(),
                    None => "n/a".to_string(),
                })]
                .iter(),
            )
            .block(block_pv_inverter)
            .wrap(true)
            .render(&mut frame, chunks[1]);

            Paragraph::new(
                [Text::raw(match &state.vebus {
                    Some(vebus) => vebus.to_string(),
                    None => "n/a".to_string(),
                })]
                .iter(),
            )
            .block(block_vebus)
            .wrap(true)
            .render(&mut frame, chunks[2]);

            Paragraph::new(
                [Text::raw(match &state.house {
                    Some(house) => house.to_string(),
                    None => "n/a".to_string(),
                })]
                .iter(),
            )
            .block(block_house)
            .wrap(true)
            .render(&mut frame, chunks[3]);
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
            Event::Tick => {
                state = reader::read(&mut context)?;
            }
        };
    }

    Ok(())
}
