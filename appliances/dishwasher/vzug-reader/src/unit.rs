use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Kwh(pub f64);

#[derive(Debug, Serialize)]
pub struct Liter(pub f64);
