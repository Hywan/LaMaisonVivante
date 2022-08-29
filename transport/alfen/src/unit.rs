use serde::Serialize;
use std::fmt;

pub trait Unit {
    fn to_degree(&self) -> Degree;

    fn to_amp(&self) -> Amp;

    fn to_volt(&self) -> Volt;

    fn to_watt(&self) -> Watt;

    fn to_watt_hour(&self) -> WattHour;

    fn to_hertz(&self) -> Hertz;
}

impl Unit for f32 {
    fn to_degree(&self) -> Degree {
        Degree(*self)
    }

    fn to_amp(&self) -> Amp {
        Amp(*self)
    }

    fn to_volt(&self) -> Volt {
        Volt(*self)
    }

    fn to_watt(&self) -> Watt {
        Watt(*self)
    }

    fn to_watt_hour(&self) -> WattHour {
        WattHour(*self)
    }

    fn to_hertz(&self) -> Hertz {
        Hertz(*self)
    }
}

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
    };
}

unit!(Degree, "{}°C");
unit!(Amp, "{}A");
unit!(Volt, "{}V");
unit!(Watt, "{}W");
unit!(WattHour, "{}Wh");
unit!(Hertz, "{}Hz");
