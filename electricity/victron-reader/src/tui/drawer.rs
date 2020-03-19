use crate::{state::*, tui::Application};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline, Text, Widget},
};

pub(crate) trait Drawer {
    fn draw(
        application: &Application,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
        main_panel: Rect,
    );
}

impl Drawer for Battery {
    fn draw(
        application: &Application,
        mut frame: &mut Frame<CrosstermBackend<Stdout>>,
        battery_panel: Rect,
    ) {
        let state = application.last_state();

        let mut block = Block::default().title("Battery").borders(Borders::ALL);
        block.render(&mut frame, battery_panel);

        if let Some(battery) = &state.battery {
            let rows = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(20), // state of charge
                        Constraint::Percentage(20), // power
                        Constraint::Percentage(20), // voltage
                        Constraint::Percentage(20), // temperature
                        Constraint::Percentage(20), // health
                    ]
                    .as_ref(),
                )
                .split(block.inner(battery_panel));

            Gauge::default()
                .label(&format!("state of charge {}", battery.state_of_charge))
                .style(match battery.state {
                    BatteryState::Idle => Style::default()
                        .fg(Color::Rgb(255, 255, 135))
                        .bg(Color::Black),
                    BatteryState::Discharging => Style::default()
                        .fg(Color::Rgb(255, 149, 122))
                        .bg(Color::White),
                    BatteryState::Charging => Style::default()
                        .fg(Color::Rgb(174, 255, 127))
                        .bg(Color::Black),
                })
                .percent(battery.state_of_charge.into())
                .render(&mut frame, rows[0]);

            Paragraph::new([Text::raw(format!("\npower {}", battery.ongoing_power))].iter())
                .alignment(Alignment::Center)
                .wrap(true)
                .render(&mut frame, rows[1]);

            Paragraph::new([Text::raw(format!("\nvoltage {}", battery.voltage))].iter())
                .wrap(true)
                .render(&mut frame, rows[2]);

            Paragraph::new([Text::raw(format!("\ntemperature {}", battery.temperature))].iter())
                .wrap(true)
                .render(&mut frame, rows[3]);

            Gauge::default()
                .label(&format!("health {}", battery.health))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .percent(battery.health.into())
                .render(&mut frame, rows[4]);
        }
    }
}

impl Drawer for PvInverter {
    fn draw(
        application: &Application,
        mut frame: &mut Frame<CrosstermBackend<Stdout>>,
        pv_inverter_panel: Rect,
    ) {
        let mut block = Block::default().title("PV Inverter").borders(Borders::ALL);
        block.render(&mut frame, pv_inverter_panel);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(25), // L1
                    Constraint::Percentage(25), // L2
                    Constraint::Percentage(25), // L3
                    Constraint::Percentage(25), // total
                ]
                .as_ref(),
            )
            .split(block.inner(pv_inverter_panel));

        macro_rules! sparkline {
            ($label:expr, $phase:ident, $field:ident, $row:expr) => {
                Sparkline::default()
                    .block(Block::default().title($label).borders(Borders::LEFT))
                    .data(
                        application
                            .states
                            .iter()
                            .map(|state| match &state.pv_inverter {
                                Some(pv_inverter) => pv_inverter.$phase.$field.into(),
                                None => 0,
                            })
                            .collect::<Vec<u64>>()
                            .as_slice(),
                    )
                    .style(Style::default().fg(Color::Yellow))
                    .render(&mut frame, $row);
            };
        }

        {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(columns[0]);

            sparkline!("L1 Voltage", l1, voltage, rows[0]);
            sparkline!("L1 Current", l1, current, rows[1]);
            sparkline!("L1 Power", l1, power, rows[2]);
        }

        {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(columns[1]);

            sparkline!("L2 Voltage", l2, voltage, rows[0]);
            sparkline!("L2 Current", l2, current, rows[1]);
            sparkline!("L2 Power", l2, power, rows[2]);
        }

        {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(columns[2]);

            sparkline!("L3 Voltage", l3, voltage, rows[0]);
            sparkline!("L3 Current", l3, current, rows[1]);
            sparkline!("L3 Power", l3, power, rows[2]);
        }

        {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(columns[3]);

            if let Some(pv_inverter) = &application.last_state().pv_inverter {
                Paragraph::new(
                    [Text::raw(
                        (pv_inverter.l1.voltage + pv_inverter.l2.voltage + pv_inverter.l3.voltage)
                            .to_string(),
                    )]
                    .iter(),
                )
                .block(
                    Block::default()
                        .title("Total voltage")
                        .borders(Borders::LEFT | Borders::RIGHT),
                )
                .wrap(true)
                .render(&mut frame, rows[0]);

                Paragraph::new(
                    [Text::raw(
                        (pv_inverter.l1.current + pv_inverter.l2.current + pv_inverter.l3.current)
                            .to_string(),
                    )]
                    .iter(),
                )
                .block(
                    Block::default()
                        .title("Total current")
                        .borders(Borders::LEFT | Borders::RIGHT),
                )
                .wrap(true)
                .render(&mut frame, rows[1]);

                Paragraph::new(
                    [Text::raw(
                        (pv_inverter.l1.power + pv_inverter.l2.power + pv_inverter.l3.power)
                            .to_string(),
                    )]
                    .iter(),
                )
                .block(
                    Block::default()
                        .title("Total power")
                        .borders(Borders::LEFT | Borders::RIGHT),
                )
                .wrap(true)
                .render(&mut frame, rows[2]);
            }
        }
    }
}

impl Drawer for Vebus {
    fn draw(
        application: &Application,
        mut frame: &mut Frame<CrosstermBackend<Stdout>>,
        vebus_panel: Rect,
    ) {
        let state = application.last_state();
        let mut block = Block::default().title("Vebus").borders(Borders::ALL);
        block.render(&mut frame, vebus_panel);

        if let Some(vebus) = &state.vebus {
            let rows = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(block.inner(vebus_panel));

            Paragraph::new([Text::raw(format!("frequency {}", vebus.frequency))].iter())
                .wrap(true)
                .render(&mut frame, rows[0]);
        }
    }
}

impl Drawer for House {
    fn draw(
        application: &Application,
        mut frame: &mut Frame<CrosstermBackend<Stdout>>,
        house_panel: Rect,
    ) {
        let mut block = Block::default().title("House").borders(Borders::ALL);
        block.render(&mut frame, house_panel);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(25), // L1
                    Constraint::Percentage(25), // L2
                    Constraint::Percentage(25), // L3
                    Constraint::Percentage(25), // total
                ]
                .as_ref(),
            )
            .split(block.inner(house_panel));

        Sparkline::default()
            .block(Block::default().title("L1").borders(Borders::LEFT))
            .data(
                application
                    .states
                    .iter()
                    .map(|state| match &state.house {
                        Some(house) => house.l1.into(),
                        None => 0,
                    })
                    .collect::<Vec<u64>>()
                    .as_slice(),
            )
            .style(Style::default().fg(Color::Yellow))
            .render(&mut frame, columns[0]);

        Sparkline::default()
            .block(Block::default().title("L2").borders(Borders::LEFT))
            .data(
                application
                    .states
                    .iter()
                    .map(|state| match &state.house {
                        Some(house) => house.l2.into(),
                        None => 0,
                    })
                    .collect::<Vec<u64>>()
                    .as_slice(),
            )
            .style(Style::default().fg(Color::Yellow))
            .render(&mut frame, columns[1]);

        Sparkline::default()
            .block(Block::default().title("L3").borders(Borders::LEFT))
            .data(
                application
                    .states
                    .iter()
                    .map(|state| match &state.house {
                        Some(house) => house.l3.into(),
                        None => 0,
                    })
                    .collect::<Vec<u64>>()
                    .as_slice(),
            )
            .style(Style::default().fg(Color::Yellow))
            .render(&mut frame, columns[2]);

        if let Some(house) = &application.last_state().house {
            Paragraph::new([Text::raw((house.l1 + house.l2 + house.l3).to_string())].iter())
                .block(
                    Block::default()
                        .title("Total power")
                        .borders(Borders::LEFT | Borders::RIGHT),
                )
                .wrap(true)
                .render(&mut frame, columns[3]);
        }
    }
}
