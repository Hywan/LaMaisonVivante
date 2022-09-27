use crate::{
    auth::Authentification,
    brand::{Brand, BrandConfiguration, Region},
    errors::Error,
    identity::{LoginClient, Tokens},
    vehicles::Vehicles,
};
use async_trait::async_trait;
use std::fmt;

pub struct Garage {
    backend: Box<dyn Backend + Send + Sync>,
}

impl fmt::Debug for Garage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Garage").finish_non_exhaustive()
    }
}

impl Garage {
    pub async fn new(
        region: Region,
        brand: Brand,
        authentification: &Authentification,
    ) -> Result<Garage, Error> {
        let backend = match (region, brand) {
            (Region::Europe, Brand::Kia) => KiaEurope::new(),
        };

        let mut this = Self {
            backend: Box::new(backend),
        };

        this.backend.login(authentification).await?;

        Ok(this)
    }

    pub async fn vehicles(&self) -> Result<Vehicles, Error> {
        self.backend.vehicles().await
    }

    //async fn open(
}

#[async_trait]
trait Backend {
    async fn login(&mut self, auth: &Authentification) -> Result<(), Error>;
    async fn vehicles(&self) -> Result<Vehicles, Error>;
}

#[derive(Debug)]
pub struct KiaEurope {
    brand: Brand,
    brand_configuration: BrandConfiguration,
    tokens: Option<Tokens>,
}

impl KiaEurope {
    pub fn new() -> Self {
        let brand = Brand::Kia;

        Self {
            brand,
            brand_configuration: BrandConfiguration::new(brand),
            tokens: None,
        }
    }
}

#[async_trait]
impl Backend for KiaEurope {
    async fn login(&mut self, auth: &Authentification) -> Result<(), Error> {
        let mut login_client = LoginClient::new(self.brand, &self.brand_configuration).await?;
        self.tokens.replace(login_client.login(auth).await?);

        Ok(())
    }

    async fn vehicles(&self) -> Result<Vehicles, Error> {
        Vehicles::new(
            self.brand,
            &self.brand_configuration,
            self.tokens.as_ref().ok_or(Error::MustBeLogged)?,
        )
        .await
    }
}
