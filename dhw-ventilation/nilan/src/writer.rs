use crate::{modbus::*, state::*};
use std::io::Result;
use tokio_modbus::prelude::*;

pub fn set_ventilation_state(
    context: &mut sync::Context,
    new_state: VentilationState,
) -> Result<()> {
    context.set_slave(Slave(1));
    context.write_single_register(VENTILATION_STATE, new_state.into())
}

pub fn toggle_ventilation(context: &mut sync::Context, current_state: &State) -> Result<()> {
    context.set_slave(Slave(1));

    match current_state.ventilation.state {
        VentilationState::Paused => {
            context.write_single_register(VENTILATION_STATE, VentilationState::Running.into())
        }

        VentilationState::Running => {
            context.write_single_register(VENTILATION_STATE, VentilationState::Paused.into())
        }
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
