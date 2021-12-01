use crate::{modbus::*, state::*};
use std::io::Result;
use tokio_modbus::prelude::*;

pub fn toggle_ventilation(context: &mut sync::Context, current_state: &State) -> Result<()> {
    match current_state.ventilation.state {
        VentilationState::Paused => context.write_single_register(VENTILATION_STATE, 0),

        VentilationState::Running => context.write_single_register(VENTILATION_STATE, 1),
    }
}

pub fn toggle_hot_water(context: &mut sync::Context, current_state: &State) -> Result<()> {
    if current_state
        .domestic_hot_water
        .anti_legionella
        .started_manually
    {
        context.write_single_register(START_ANTI_LEGIONELLA_MANUALLY, 0)
    } else {
        context.write_single_register(START_ANTI_LEGIONELLA_MANUALLY, 1)
    }
}
