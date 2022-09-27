use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read the configuration")]
    ReadConfiguration(String),

    #[error("failed to load the configuration")]
    LoadConfiguration(#[from] confy::ConfyError),
    
    #[error("failed to send or fetch an HTTP request or response")]
    Http(#[from] reqwest::Error),

    #[error("failed to log in: {0}")]
    Login(String),

    #[error("must be logged")]
    MustBeLogged,
}
