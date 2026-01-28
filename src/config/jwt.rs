use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    secret:Option<String>,
}

impl JwtConfig {
    pub fn secret(&self) -> &str {
        self.secret.as_deref().unwrap_or("12345678")
    }
}
