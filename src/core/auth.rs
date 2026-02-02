use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::LazyLock;
use std::time::Duration;

static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(|| JWT::default());

#[derive(Serialize, Debug, Clone)]
pub struct Principal {
    pub id: String,
    pub name: String,
}

impl Principal {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,

    name: String,
}

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: Cow<'static, str>,
    pub expiration: Duration,
    pub audience: Cow<'static, str>,
    pub issuer: Cow<'static, str>,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: Cow::Borrowed(crate::config::get().jwt().secret()),
            expiration: Duration::from_secs(crate::config::get().jwt().expiration()),
            audience: Cow::Borrowed(crate::config::get().jwt().audience()),
            issuer: Cow::Borrowed(crate::config::get().jwt().issuer()),
        }
    }
}

pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);

        let secret = config.secret.as_bytes();

        Self {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: config.expiration,
            audience: config.audience.to_string(),
            issuer: config.issuer.to_string(),
        }
    }

    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: principal.id,
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),

            name: principal.name,
        };

        Ok(encode(&self.header, &claims, &self.encode_secret)?)
    }

    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims: Claims = decode(token, &self.decode_secret, &self.validation)?.claims;

        let principal = Principal::new(claims.sub, claims.name);
        Ok(principal)
    }
}

impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}

pub fn get_jwt() -> &'static JWT {
    &DEFAULT_JWT
}
