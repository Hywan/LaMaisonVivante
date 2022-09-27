mod auth;
mod brand;
mod errors;
mod garage;
mod http;
mod identity;
mod units;
mod vehicles;

use crate::{
    auth::Authentification,
    brand::{Brand, Region},
    errors::Error,
    garage::Garage,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth = Authentification::new(
        "ivan@mnt.io".to_string(),
        "i_@P-niCzs3dAm!Nkmwa".to_string(),
    );

    let brand = Brand::Kia;
    let garage = Garage::new(Region::Europe, brand, &auth).await?;
    let vehicles = garage.vehicles().await?;

    dbg!(vehicles.get(0).unwrap().state().await?);

    Ok(())
}
