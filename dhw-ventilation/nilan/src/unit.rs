use serde::Serialize;
use std::{fmt, ops};

pub trait Unit {
    fn to_bool(&self) -> bool;

    fn to_percent(&self) -> Percent;

    /// Revolution per minute.
    fn to_rpm(&self) -> Rpm;

    /// Parts-per million.
    fn to_ppm(&self) -> Ppm;

    fn to_degree(&self) -> Degree;
}

impl Unit for u16 {
    fn to_bool(&self) -> bool {
        *self > 0
    }

    fn to_percent(&self) -> Percent {
        Percent(*self as f32)
    }

    fn to_rpm(&self) -> Rpm {
        Rpm(*self as f32)
    }

    fn to_ppm(&self) -> Ppm {
        Ppm(*self as f32)
    }

    fn to_degree(&self) -> Degree {
        Degree((*self as f32) / 10.0)
    }
}

macro_rules! unit {
    ($name:ident, $display:expr) => {
        #[derive(Debug, Copy, Clone, Serialize)]
        pub struct $name(pub f32);

        impl fmt::Display for $name {
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
unit!(Rpm, "{}rev/min");
unit!(Ppm, "{}ppm");
unit!(Degree, "{}°C");
