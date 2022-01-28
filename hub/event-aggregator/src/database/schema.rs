table! {
    use crate::database::enums::AirStateMapping;
    use diesel::sql_types::*;

    air (time) {
        time -> Timestamp,
        inside_humidity -> Float8,
        supplied_temperature_after_ground_coupled_heat_exchanger -> Float8,
        supplied_temperature_after_heat_recovery_exchanger -> Float8,
        extracted_temperature -> Float8,
        discharged_temperature -> Float8,
        wanted_temperature -> Float8,
        state -> Nullable<AirStateMapping>,
    }
}

table! {
    domestic_hot_water (time) {
        time -> Timestamp,
        top_of_the_tank_temperature -> Float8,
        bottom_of_the_tank_temperature -> Float8,
        wanted_temperature -> Float8,
    }
}

table! {
    electricity_consumption (time) {
        time -> Timestamp,
        house_power -> Float8,
        house_l1_power -> Float8,
        house_l2_power -> Float8,
        house_l3_power -> Float8,
    }
}

table! {
    electricity_production (time) {
        time -> Timestamp,
        l1_voltage -> Float8,
        l1_frequency -> Float8,
        l1_power -> Float8,
        l1_current -> Float8,
        l2_voltage -> Float8,
        l2_frequency -> Float8,
        l2_power -> Float8,
        l2_current -> Float8,
        l3_voltage -> Float8,
        l3_frequency -> Float8,
        l3_power -> Float8,
        l3_current -> Float8,
        voltage -> Float8,
        frequency -> Float8,
        power -> Float8,
        current -> Float8,
    }
}

table! {
    electricity_storage (time) {
        time -> Timestamp,
        ongoing_power -> Float8,
        temperature -> Float8,
        state_of_charge -> Float8,
        voltage -> Float8,
    }
}

allow_tables_to_appear_in_same_query!(
    air,
    domestic_hot_water,
    electricity_consumption,
    electricity_production,
    electricity_storage,
);
