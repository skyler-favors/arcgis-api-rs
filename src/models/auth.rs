use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: SecretString,
    pub expires: i64,
    pub ssl: bool,
}
