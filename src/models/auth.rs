use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: SecretString,
    pub expires: i64,
    #[allow(dead_code)]
    ssl: bool,
}
