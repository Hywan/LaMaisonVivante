#[derive(Debug, Clone)]
pub struct Authentification {
    pub username: String,
    pub password: String,
    pub pin: Option<String>,
}

impl Authentification {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            pin: None,
        }
    }

    /*
    pub fn pin(mut self, pin: String) -> Self {
        self.pin = Some(pin);

        self
    }
    */
}
