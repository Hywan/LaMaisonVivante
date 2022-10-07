use serde::Serialize;
use std::{fmt, ops};

pub trait Unit {
    fn to_percent(&self) -> Percent;
    fn to_volt(&self) -> Volt;
    fn to_amp(&self) -> Amp;
    fn to_watt(&self) -> Watt;
    fn to_kwh(&self) -> KWh;
    fn to_degree(&self) -> Degree;
    fn to_hertz(&self) -> Hertz;
}

macro_rules! impl_unit {
    ($for:ty) => {
        impl Unit for $for {
            fn to_percent(&self) -> Percent {
                Percent((*self as f32) / 10.0)
            }

            fn to_volt(&self) -> Volt {
                Volt((*self as f32) / 100.0)
            }

            fn to_amp(&self) -> Amp {
                Amp((*self) as f32)
            }

            fn to_watt(&self) -> Watt {
                Watt((*self) as f32)
            }

            fn to_kwh(&self) -> KWh {
                KWh(*self as f32)
            }

            fn to_degree(&self) -> Degree {
                Degree((*self as f32) / 10.0)
            }

            fn to_hertz(&self) -> Hertz {
                Hertz((*self as f32) / 100.0)
            }
        }
    };
}

impl_unit!(u16);
impl_unit!(i32);

macro_rules! unit {
    ($name:ident, $display:expr) => {
        #[derive(Copy, Clone, Serialize)]
        pub struct $name(pub f32);

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, $display, self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, $display, self.0)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(0.0)
            }
        }

        impl ops::Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self(self.0 + other.0)
            }
        }

        impl ops::Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self(self.0 - other.0)
            }
        }

        impl From<$name> for u16 {
            fn from(unit: $name) -> Self {
                unit.0.ceil() as _
            }
        }

        impl From<$name> for u64 {
            fn from(unit: $name) -> Self {
                unit.0.ceil() as _
            }
        }
    };
}

unit!(Percent, "{}%");
unit!(Volt, "{}V");
unit!(Amp, "{}A");
unit!(Watt, "{}W");
unit!(KWh, "{}kWh");
unit!(Degree, "{}°C");
unit!(Hertz, "{}Hz");
