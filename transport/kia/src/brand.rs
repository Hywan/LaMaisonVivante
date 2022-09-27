#[derive(Debug, Copy, Clone)]
pub enum Region {
    Europe,
    //Canada,
    //USA,
}

#[derive(Debug, Copy, Clone)]
pub enum Brand {
    Kia,
}

impl Brand {
    pub fn as_id(&self) -> &'static str {
        match self {
            Self::Kia => "kia",
        }
    }

    pub fn application_id(&self) -> &'static str {
        match self {
            Self::Kia => "e7bcd186-a5fd-410d-92cb-6876a42288bd",
        }
    }

    pub fn client_id(&self) -> &'static str {
        match self {
            Self::Kia => "fdc85c00-0a2f-4c64-bcb4-2cfb1500730a",
        }
    }
}

#[derive(Debug)]
pub struct BrandConfiguration {
    pub uri: String,
    pub basic_token: String,
    pub auth_client_id: String,
    pub auth_url_format: String,
    pub basic_authorization_password: String,
}

impl BrandConfiguration {
    pub fn new(brand: Brand) -> Self {
        match brand {
            Brand::Kia =>
                Self {
                    uri: "https://prd.eu-ccapi.kia.com:8080".to_string(),
                    basic_token: "ZmRjODVjMDAtMGEyZi00YzY0LWJjYjQtMmNmYjE1MDA3MzBhOnNlY3JldA==".to_string(),
                    auth_client_id: "f4d531c7-1043-444d-b09a-ad24bd913dd4".to_string(),
                    auth_url_format: "https://eu-account.kia.com/auth/realms/eukiaidm/protocol/openid-connect/auth?client_id={client_id}&scope=openid%20profile%20email%20phone&response_type=code&hkid_session_reset=true&redirect_uri={uri}/api/v1/user/integration/redirect/login&ui_locales=en&state={service_id}:{user_id}".to_string(),
                    basic_authorization_password: "secret".to_string(),
                }
        }
    }
}
