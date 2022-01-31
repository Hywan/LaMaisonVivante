use crate::{modbus::*, state::*, unit::*};
use std::{convert::TryInto, io::Result};
use tokio_modbus::prelude::*;

fn read_holding_register(context: &mut sync::Context, address: u16) -> Result<u16> {
    Ok(context.read_holding_registers(address, 1)?[0])
}

pub fn read_ventilation(mut context: &mut sync::Context) -> Result<Ventilation> {
    context.set_slave(Slave(1));

    Ok(Ventilation {
        mode: read_holding_register(&mut context, VENTILATION_MODE)?
            .try_into()
            .map_err(|e| unreachable!(e))
            .unwrap(),
        state: read_holding_register(&mut context, VENTILATION_STATE)?
            .try_into()
            .map_err(|e| unreachable!(e))
            .unwrap(),
        air_throughput: AirThroughput {
            supplied_air_fan_speed: read_holding_register(&mut context, SUPPLIED_AIR_FAN_SPEED)?
                .to_rpm(),
            extracted_air_fan_speed: read_holding_register(&mut context, EXTRACTED_AIR_FAN_SPEED)?
                .to_rpm(),
        },
        inside_air_humidity: read_holding_register(&mut context, INSIDE_AIR_HUMIDITY)?.to_percent(),
        inside_co2_level: read_holding_register(&mut context, INSIDE_CO2_LEVEL)?.to_ppm(),
        temperatures: AirTemperatures {
            supplied_air_after_ground_coupled_heat_exchanger: read_holding_register(
                &mut context,
                SUPPLIED_AIR_TEMPERATURE_AFTER_GROUND_COUPLED_HEAT_EXCHANGER,
            )?
            .to_degree(),
            supplied_air_after_heat_recovery_exchanger: read_holding_register(
                &mut context,
                SUPPLIED_AIR_TEMPERATURE_AFTER_HEAT_RECOVERY_EXCHANGER,
            )?
            .to_degree(),
            extracted_air: read_holding_register(&mut context, EXTRACTED_AIR_TEMPERATURE)?
                .to_degree(),
            discharged_air: read_holding_register(&mut context, DISCHARGED_AIR_TEMPERATURE)?
                .to_degree(),
            wanted_inside_air: read_holding_register(&mut context, WANTED_INSIDE_AIR_TEMPERATURE)?
                .to_degree(),
        },
    })
}

pub fn read_domestic_hot_water(mut context: &mut sync::Context) -> Result<DomesticHotWater> {
    context.set_slave(Slave(1));

    Ok(DomesticHotWater {
        anti_legionella: AntiLegionella {
            started_manually: read_holding_register(&mut context, START_ANTI_LEGIONELLA_MANUALLY)?
                .to_bool(),
            frequency: match read_holding_register(&mut context, AUTOMATIC_ANTI_LEGIONELLA)? {
                0 => AntiLegionellaFrequency::Off,
                1 => AntiLegionellaFrequency::Weekly,
                2 => AntiLegionellaFrequency::Monthly,
                v @ _ => unreachable!("Unrecognized anti-legionella frequency (`{}`).", v),
            },
            day: read_holding_register(&mut context, DAY_FOR_ANTI_LEGIONELLA)?,
            hour: read_holding_register(&mut context, TIME_FOR_ANTI_LEGIONELLA)?,
        },
        storage_temperatures: StorageHotWaterTemperatures {
            top_of_the_tank: read_holding_register(
                &mut context,
                TOP_TEMPERATURE_IN_DOMESTIC_HOT_WATER_TANK,
            )?
            .to_degree(),
            bottom_of_the_tank: read_holding_register(
                &mut context,
                BOTTOM_TEMPERATURE_IN_DOMESTIC_HOT_WATER_TANK,
            )?
            .to_degree(),
            wanted: read_holding_register(&mut context, WANTED_HOT_WATER_TEMPERATURE)?.to_degree(),
        },
    })
}

pub fn read(mut context: &mut sync::Context) -> Result<State> {
    Ok(State {
        ventilation: read_ventilation(&mut context)?,
        domestic_hot_water: read_domestic_hot_water(&mut context)?,
    })
}
