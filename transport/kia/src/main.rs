use async_trait::async_trait;
use std::fmt;

mod auth;
mod brand;
mod errors;
mod http;
mod identity;

use auth::Authentification;
use brand::{Brand, BrandConfiguration, Region};
use errors::Error;
use identity::{AuthorizedClient, DeviceId};

#[async_trait]
trait Backend {
    async fn login(&self, auth: &Authentification) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct KiaEurope;

#[async_trait]
impl Backend for KiaEurope {
    async fn login(&self, auth: &Authentification) -> Result<(), Error> {
        dbg!("here");
        let brand_configuration = BrandConfiguration::new(Brand::Kia);
        let device_id = DeviceId::new(Brand::Kia, &brand_configuration).await?;
        dbg!(&device_id);

        let authorized_client = AuthorizedClient::new(Brand::Kia, &brand_configuration).await?;
        authorized_client.login(auth).await?;

        unimplemented!()
    }
}

pub struct Garage<'a> {
    region: Region,
    brand: Brand,
    authentification: &'a Authentification,
    backend: Box<dyn Backend>,
}

impl<'a> fmt::Debug for Garage<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Garage")
            .field("region", &self.region)
            .field("brand", &self.brand)
            .field("authentification", self.authentification)
            .finish_non_exhaustive()
    }
}

impl<'a> Garage<'a> {
    async fn new(
        region: Region,
        brand: Brand,
        authentification: &'a Authentification,
    ) -> Result<Garage<'a>, Error> {
        let backend = match (region, brand) {
            (Region::Europe, Brand::Kia) => KiaEurope,
            _ => unimplemented!("region and brand not supported"),
        };

        let this = Self {
            region,
            brand,
            authentification,
            backend: Box::new(backend),
        };

        this.backend.login(&this.authentification).await?;

        Ok(this)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth = Authentification::new(
        "ivan@mnt.io".to_string(),
        "i_@P-niCzs3dAm!Nkmwa".to_string(),
    );

    let brand = Brand::Kia;
    let garage = Garage::new(Region::Europe, brand, &auth).await?;

    Ok(())
}
