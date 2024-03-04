use crate::{
    auth::Authentification,
    brand::{Brand, BrandConfiguration},
    errors::Error,
    http::Client,
};
use base64::prelude::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    ops::Deref,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const REGISTER_URL: &'static str = "/api/v1/spa/notifications/register";
const USER_AUTHORIZE_URL: &'static str = "/api/v1/user/oauth2/authorize";
const USER_REDIRECT_URL: &'static str = "/api/v1/user/oauth2/redirect";
const USER_LANGUAGE_URL: &'static str = "/api/v1/user/language";
const USER_SILENT_LOGIN_URL: &'static str = "/api/v1/user/silentsignin";
const USER_TOKEN_URL: &'static str = "/api/v1/user/oauth2/token";
const USER_TOKEN_REDIRECT_URL: &'static str = "/api/v1/user/oauth2/redirect";

#[repr(transparent)]
#[derive(Debug)]
pub struct Stamp(String);

impl Stamp {
    pub fn new(brand: Brand) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let raw_data = format!("{app_id}:{now}", app_id = brand.application_id());
        let cfb = BASE64_STANDARD.decode(brand.cfb()).unwrap();

        let bytes = cfb
            .into_iter()
            .zip(raw_data.into_bytes().into_iter())
            .map(|(b1, b2)| b1 ^ b2)
            .collect::<Vec<u8>>();

        Self(BASE64_STANDARD.encode(bytes))
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn to_string(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub struct DeviceId {
    pub id: String,
    pub stamp: String,
}

impl DeviceId {
    pub async fn new(
        brand: Brand,
        brand_configuration: &BrandConfiguration,
    ) -> Result<Self, Error> {
        let stamp = Stamp::new(brand);

        let push_reg_id = (0..64)
            .into_iter()
            .map(|_| rand::random::<char>())
            .collect::<String>();

        let mut http_request_headers = reqwest::header::HeaderMap::with_capacity(3);
        http_request_headers.insert("ccsp-service-id", brand.client_id().parse().unwrap());
        http_request_headers.insert(
            "ccsp-application-id",
            brand.application_id().parse().unwrap(),
        );
        http_request_headers.insert("Content-Type", "application/json".parse().unwrap());
        http_request_headers.insert("Stamp", stamp.as_str().parse().unwrap());

        let uuid = Uuid::new_v4().to_string();
        let mut http_request_body = HashMap::with_capacity(3);
        http_request_body.insert("pushRegId", push_reg_id.as_str());
        http_request_body.insert("pushType", "APNS");
        http_request_body.insert("uuid", &uuid);

        #[derive(Debug, Deserialize)]
        struct InnerDeviceId {
            #[serde(rename = "deviceId")]
            id: String,
        }

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "resMsg")]
            result_message: InnerDeviceId,
        }

        Ok(Self {
            id: Client::post(format!(
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
            .result_message
            .id,
            stamp: stamp.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub device_id: String,
    pub stamp: String,
}

#[derive(Debug)]
pub struct LoginClient<'a> {
    client: reqwest::Client,
    brand: Brand,
    brand_configuration: &'a BrandConfiguration,
}

impl<'a> LoginClient<'a> {
    pub async fn new(
        brand: Brand,
        brand_configuration: &'a BrandConfiguration,
    ) -> Result<LoginClient<'a>, Error> {
        let client = Client::new().cookie_store(true).redirect(true).build()?;

        // Authorization.
        {
            // Populate the cookie jar.
            client.get(format!(
                "{url}{path}?response_type=code&state=test&client_id={client_id}&redirect_uri={url}{redirect}&lang=en",
                url = brand_configuration.uri,
                path = USER_AUTHORIZE_URL,
                client_id = brand.client_id(),
                redirect = USER_REDIRECT_URL,
            ))
            .send()
            .await
            .map_err(Error::Http)?;
        }

        // Language.
        {
            let mut http_request_body = HashMap::with_capacity(1);
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
            brand,
            brand_configuration,
        })
    }

    pub async fn login(&mut self, auth: &Authentification) -> Result<Tokens, Error> {
        #[derive(Debug, Deserialize)]
        struct IntegrationInfo {
            #[serde(rename = "userId")]
            user_id: String,

            #[serde(rename = "serviceId")]
            service_id: String,
        }

        // Step 1, get integration info.
        let integration_info: IntegrationInfo = self
            .client
            .get(format!(
                "{url}{path}",
                url = self.brand_configuration.uri,
                path = "/api/v1/user/integrationinfo",
            ))
            .send()
            .await
            .map_err(Error::Http)?
            .json()
            .await
            .map_err(Error::Http)?;

        // Step 2, fetch page to read form's action
        let form_url = {
            let html_response = self
                .client
                .get(
                    self.brand_configuration
                        .auth_url_format
                        .replace("{client_id}", &self.brand_configuration.auth_client_id)
                        .replace("{uri}", &self.brand_configuration.uri)
                        .replace("{service_id}", &integration_info.service_id)
                        .replace("{user_id}", &integration_info.user_id),
                )
                .send()
                .await
                .map_err(Error::Http)?
                .text()
                .await
                .map_err(Error::Http)?;

            let html_document = scraper::Html::parse_document(&html_response);
            let form_selector = scraper::Selector::parse("form[action]").unwrap();

            html_document
                .select(&form_selector)
                .next()
                .ok_or_else(|| Error::Login("Cannot find the form's action".to_string()))?
                .value()
                .attr("action")
                .unwrap()
                .to_string()
        };

        // Step 3, send data to the form's action received in Step 2.
        {
            let http_form_data: [(&str, &str); 4] = [
                ("username", &auth.username),
                ("password", &auth.password),
                ("credentialId", ""),
                ("rememberMe", "on"),
            ];

            self.client
                .post(form_url)
                .form(&http_form_data)
                .send()
                .await
                .map_err(Error::Http)?;
        }

        // Step 4, silent sign in and get the code!
        let code = {
            let mut http_request_headers = reqwest::header::HeaderMap::with_capacity(1);
            http_request_headers.insert("ccsp-service-id", self.brand.client_id().parse().unwrap());

            let mut http_request_body = HashMap::with_capacity(1);
            http_request_body.insert("intUserId", "0");

            #[derive(Debug, Deserialize)]
            struct Response {
                #[serde(rename = "redirectUrl")]
                redirect_url: String,
            }

            let response = self
                .client
                .post(format!(
                    "{url}{path}",
                    url = self.brand_configuration.uri,
                    path = USER_SILENT_LOGIN_URL,
                ))
                .headers(http_request_headers)
                .json(&http_request_body)
                .send()
                .await
                .map_err(Error::Http)?
                .json::<Response>()
                .await
                .map_err(Error::Http)?;

            reqwest::Url::parse(&response.redirect_url)
                .map_err(|_| {
                    Error::Login("failed to parse the `code` from the `redirect_url`".to_string())
                })?
                .query_pairs()
                .find_map(|(name, value)| {
                    if name == "code" {
                        Some(value.to_string())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    Error::Login("`code` is missing from the `redirect_url`".to_string())
                })?
        };

        // Step 5, get the access token!
        let (access_token, refresh_code) = {
            let http_form_data: [(&str, &str); 3] = [
                ("grant_type", "authorization_code"),
                (
                    "redirect_uri",
                    &format!(
                        "{url}{path}",
                        url = self.brand_configuration.uri,
                        path = USER_TOKEN_REDIRECT_URL
                    ),
                ),
                ("code", &code),
            ];

            #[derive(Debug, Deserialize)]
            struct Response {
                token_type: String,
                access_token: String,
                refresh_token: String,
            }

            let response = Client::post(format!(
                "{url}{path}",
                url = self.brand_configuration.uri,
                path = USER_TOKEN_URL
            ))?
            .basic_auth(
                self.brand.client_id(),
                Some(&self.brand_configuration.basic_authorization_password),
            )
            .form(&http_form_data)
            .send()
            .await
            .map_err(Error::Http)?
            .json::<Response>()
            .await
            .map_err(Error::Http)?;

            (
                format!(
                    "{token_type} {access_token}",
                    token_type = response.token_type,
                    access_token = response.access_token
                ),
                response.refresh_token,
            )
        };

        // Step 6, get device ID.
        let device_id = DeviceId::new(self.brand, self.brand_configuration).await?;

        // Step 7, get the refresh token!
        let refresh_token = {
            let mut http_request_headers = reqwest::header::HeaderMap::with_capacity(1);
            http_request_headers.insert("Stamp", device_id.stamp.parse().unwrap());

            let http_form_data: [(&str, &str); 3] = [
                ("grant_type", "refresh_token"),
                ("redirect_uri", "https://www.getpostman.com/oauth2/callback"),
                ("refresh_token", &refresh_code),
            ];

            #[derive(Debug, Deserialize)]
            struct Response {
                token_type: String,
                access_token: String,
            }

            let response = Client::post(format!(
                "{url}{path}",
                url = self.brand_configuration.uri,
                path = USER_TOKEN_URL
            ))?
            .basic_auth(
                self.brand.client_id(),
                Some(&self.brand_configuration.basic_authorization_password),
            )
            .headers(http_request_headers)
            .form(&http_form_data)
            .send()
            .await
            .map_err(Error::Http)?
            .json::<Response>()
            .await
            .map_err(Error::Http)?;

            format!(
                "{token_type} {access_token}",
                token_type = response.token_type,
                access_token = response.access_token
            )
        };

        Ok(Tokens {
            access_token,
            refresh_token,
            device_id: device_id.id,
            stamp: device_id.stamp,
        })
    }
}

impl<'a> Deref for LoginClient<'a> {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
