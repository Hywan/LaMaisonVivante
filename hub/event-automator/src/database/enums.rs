use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, Clone)]
pub enum AirState {
    Paused,
    Running,
}

impl Default for AirState {
    fn default() -> Self {
        Self::Paused
    }
}
