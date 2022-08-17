use crate::errors::Error;

#[derive(Debug)]
pub struct Client {
    inner: reqwest::ClientBuilder,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::builder()
                .user_agent("lmv/1.0.0")
                .danger_accept_invalid_certs(true),
        }
    }

    pub fn cookie_store(mut self, enable: bool) -> Self {
        self.inner = self.inner.cookie_store(enable);

        self
    }

    pub fn build(self) -> Result<reqwest::Client, Error> {
        self.inner.build().map_err(Error::Http)
    }

    pub fn get<U>(url: U) -> Result<reqwest::RequestBuilder, Error>
    where
        U: reqwest::IntoUrl,
    {
        Ok(Self::new().build()?.get(url))
    }

    pub fn post<U>(url: U) -> Result<reqwest::RequestBuilder, Error>
    where
        U: reqwest::IntoUrl,
    {
        Ok(Self::new().build()?.post(url))
    }
}
