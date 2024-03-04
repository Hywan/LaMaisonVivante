pub use crate::database::enums::AirState as VentilationState;
use crate::database::models::*;
use crate::events::Event;
use chrono::prelude::*;
use diesel::{prelude::*, sql_query};

pub struct Context {
    pub database_connection: PgConnection,
}

// That's unsafe. Really.
unsafe impl Send for Context {}

pub trait UpdateState {
    fn update(&self, context: &Context, new_events: &mut Vec<Event>) -> Self;
}

const HOME_LATITUDE: f64 = 46.78657339107215;
const HOME_LONGITUDE: f64 = 6.806581635522576;

#[derive(Debug, PartialEq)]
pub enum SunPeriod {
    Day,
    Night,
}

#[derive(Debug)]
pub struct Sun {
    pub period: SunPeriod,
}

impl Default for Sun {
    fn default() -> Self {
        Self {
            period: SunPeriod::Day,
        }
    }
}

impl UpdateState for Sun {
    fn update(&self, _context: &Context, new_events: &mut Vec<Event>) -> Self {
        let now: DateTime<Local> = Local::now();

        let (sunrise, sunset) = sunrise::sunrise_sunset(
            HOME_LATITUDE,
            HOME_LONGITUDE,
            now.year(),
            now.month(),
            now.day(),
        );

        let now_utc_timestamp = DateTime::<Utc>::from(now).timestamp();

        let next_state = Self {
            period: if sunrise <= now_utc_timestamp && now_utc_timestamp <= sunset {
                SunPeriod::Day
            } else {
                SunPeriod::Night
            },
        };

        if self.period != next_state.period {
            new_events.push(Event::SunPeriodChange);
        }

        next_state
    }
}

#[derive(Debug, Default)]
pub struct Ventilation {
    _state: VentilationState,
}

impl UpdateState for Ventilation {
    fn update(&self, context: &Context, new_events: &mut Vec<Event>) -> Self {
        let result = sql_query("SELECT * FROM air ORDER BY time DESC LIMIT 1")
            .load::<Air>(&context.database_connection)
            .expect("Failed to load `air` latest entry");

        new_events.push(Event::VentilationStatePersist);

        Self {
            _state: result[0].state.clone().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub sun: Sun,
    pub ventilation: Ventilation,
}

impl UpdateState for State {
    fn update(&self, context: &Context, new_events: &mut Vec<Event>) -> Self {
        Self {
            sun: self.sun.update(context, new_events),
            ventilation: self.ventilation.update(context, new_events),
        }
    }
}
