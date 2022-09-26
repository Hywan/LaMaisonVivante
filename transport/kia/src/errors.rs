use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to send or fetch an HTTP request or response")]
    Http(#[from] reqwest::Error),

    #[error("failed to log in: {0}")]
    Login(String),
}
