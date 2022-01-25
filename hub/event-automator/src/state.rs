use crate::events::Event;
use chrono::prelude::*;
use sunrise;

pub trait UpdateState {
    fn update(&self, new_events: &mut Vec<Event>) -> Self;
}

const HOME_LATITUDE: f64 = 46.78657339107215;
const HOME_LONGITUDE: f64 = 6.806581635522576;

#[derive(Debug, PartialEq)]
pub enum SunPeriod {
    Day,
    Night,
}

#[derive(Debug)]
pub struct SunState {
    pub period: SunPeriod,
}

impl Default for SunState {
    fn default() -> Self {
        Self {
            period: SunPeriod::Day,
        }
    }
}

impl UpdateState for SunState {
    fn update(&self, new_events: &mut Vec<Event>) -> Self {
        let now: DateTime<Local> = Local::now();

        let (sunrise, sunset) = sunrise::sunrise_sunset(
            HOME_LATITUDE,
            HOME_LONGITUDE,
            now.year(),
            now.month(),
            now.day(),
        );

        let now_utc: DateTime<Utc> = DateTime::from(now);
        let now_utc_timestamp = now_utc.timestamp();

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
pub struct State {
    pub sun: SunState,
}

impl UpdateState for State {
    fn update(&self, new_events: &mut Vec<Event>) -> Self {
        Self {
            sun: self.sun.update(new_events),
        }
    }
}
