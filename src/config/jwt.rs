use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    secret: Option<String>,
    expiration: Option<u64>,
    audience: Option<String>,
    issuer: Option<String>,
}

impl JwtConfig {
    pub fn secret(&self) -> &str {
        self.secret.as_deref().unwrap_or("12345678")
    }

    pub fn expiration(&self) -> u64 {
        self.expiration.unwrap_or(3600)
    }

    pub fn audience(&self) -> &str {
        self.audience.as_deref().unwrap_or("audience")
    }

    pub fn issuer(&self) -> &str {
        self.issuer.as_deref().unwrap_or("issuer")
    }
}
