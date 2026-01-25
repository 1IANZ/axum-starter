use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    name: Option<String>,
    schema: Option<String>,
}

impl DatabaseConfig {
    pub fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("postgres")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("123456")
    }

    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("postgres")
    }

    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}
