/// The events this program can fire.
#[derive(Debug)]
pub enum Event {
    /// The sun has either risen or set.
    SunPeriodChange,

    /// The ventilation has an issue: If we stop it, it actually
    /// pauses it, and it starts over a couple of minutes later. The
    /// only canonical way to stop it is to start an emergency, but we
    /// cannot do that programatically. So this program will fire the
    /// following event, so that we can “refresh” the state of the
    /// ventilation programmatically.
    VentilationStatePersist,
}
