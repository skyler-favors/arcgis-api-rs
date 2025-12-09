use config::Config;
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub portal_root: String,
    pub portal_apps_root: String,
    pub services_root: String,
    pub client_id: String,
    pub client_secret: SecretString,
    pub token_expiration: String,
    pub test_token: Option<SecretString>,
    pub test_user_name: Option<String>,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    dotenv::dotenv().ok();

    //let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    //let config_dir = base_path.join("config");
    // let env: Environment = std::env::var("APP_ENV")
    //     .unwrap_or_else(|_| "local".into())
    //     .try_into()
    //     .expect("Failed to parse APP_ENV");
    // info!("Running with env: {}", env.as_str());
    //let env_file = format!("{}.toml", env.as_str());

    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
