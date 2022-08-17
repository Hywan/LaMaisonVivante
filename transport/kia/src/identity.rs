use crate::{
    auth::Authentification,
    brand::{Brand, BrandConfiguration},
    errors::Error,
    http::Client,
};
use serde::Deserialize;
use std::{collections::HashMap, ops::Deref};
use uuid::Uuid;

const REGISTER_URL: &'static str = "/api/v1/spa/notifications/register";
const USER_AUTHORIZE_URL: &'static str = "/api/v1/user/oauth2/authorize";
const USER_REDIRECT_URL: &'static str = "/api/v1/user/oauth2/redirect";
const USER_LANGUAGE_URL: &'static str = "/api/v1/user/language";
const USER_LOGIN_URL: &'static str = "/api/v1/user/signin";

#[derive(Debug, Deserialize)]
pub struct Stamps {
    pub stamps: Vec<String>,
    pub generated: String, // a date, let's keep it as a string for now.
    pub frequency: u64,
}

impl Stamps {
    pub async fn new(brand: Brand) -> Result<Self, Error> {
        let url = format!(
            "https://raw.githubusercontent.com/neoPix/bluelinky-stamps/master/{brand_id}-{brand_application_id}.v2.json",
            brand_id = brand.as_id(),
            brand_application_id = brand.application_id(),
        );

        Client::get(url)?
            .send()
            .await
            .map_err(Error::Http)?
            .json()
            .await
            .map_err(Error::Http)
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceId {
    #[serde(rename = "deviceId")]
    pub id: String,
}

impl DeviceId {
    pub async fn new(
        brand: Brand,
        brand_configuration: &BrandConfiguration,
    ) -> Result<Self, Error> {
        let stamps = Stamps::new(brand).await?;
        let stamp = &stamps.stamps[0];

        let mut http_request_headers = reqwest::header::HeaderMap::with_capacity(3);
        http_request_headers.insert("ccsp-service-id", brand.client_id().parse().unwrap());
        http_request_headers.insert("Content-Type", "application/json".parse().unwrap());
        http_request_headers.insert("Stamp", stamp.parse().unwrap());

        let uuid = Uuid::new_v4().to_string();
        let mut http_request_body = HashMap::new();
        http_request_body.insert("pushRegId", "1");
        http_request_body.insert("pushType", "GCM");
        http_request_body.insert("uuid", &uuid);

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "resMsg")]
            result_message: DeviceId,
        }

        Ok(Client::post(format!(
            "{url}{path}",
            url = brand_configuration.uri,
            path = REGISTER_URL,
        ))?
        .headers(http_request_headers)
        .json(&http_request_body)
        .send()
        .await
        .map_err(Error::Http)?
        .json::<Response>()
        .await
        .map_err(Error::Http)?
        .result_message)
    }
}

#[derive(Debug)]
pub struct AuthorizedClient<'a> {
    client: reqwest::Client,
    brand_configuration: &'a BrandConfiguration,
}

impl<'a> AuthorizedClient<'a> {
    pub async fn new(
        brand: Brand,
        brand_configuration: &'a BrandConfiguration,
    ) -> Result<AuthorizedClient<'a>, Error> {
        let client = Client::new().cookie_store(true).build()?;

        // Authorization.
        {
            let r = client.get(format!(
                "{url}{path}?response_type=code&state=test&client_id={client_id}&redirect_uri={url}/{redirect}&lang=en",
                url = brand_configuration.uri,
                path = USER_AUTHORIZE_URL,
                client_id = brand.client_id(),
                redirect = USER_REDIRECT_URL,
            ))
            .send()
            .await
            .map_err(Error::Http)?;

            for c in r.cookies() {
                dbg!(&c);
            }
        }

        // Language.
        {
            let mut http_request_body = HashMap::new();
            http_request_body.insert("lang", "en");

            client
                .post(format!(
                    "{url}{path}",
                    url = brand_configuration.uri,
                    path = USER_LANGUAGE_URL,
                ))
                .json(&http_request_body)
                .send()
                .await
                .map_err(Error::Http)?;
        }

        Ok(Self {
            client,
            brand_configuration,
        })
    }

    pub async fn login(&self, auth: &Authentification) -> Result<String, Error> {
        let mut http_request_body = HashMap::new();
        http_request_body.insert("email", &auth.username);
        http_request_body.insert("password", &auth.password);

        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "redirectUrl")]
            redirect_url: String,
        }

        let response = dbg!(self.client
            .post(format!(
                "{url}{path}",
                url = self.brand_configuration.uri,
                path = USER_LOGIN_URL,
            ))
            .json(&http_request_body))
            .send()
            .await
            .map_err(Error::Http)?
            ;
        /*
            .json::<Response>()
            .await
            .map_err(Error::Http)?;
        */

        dbg!(&response.text().await.unwrap());

        unimplemented!()
    }
}

impl<'a> Deref for AuthorizedClient<'a> {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
