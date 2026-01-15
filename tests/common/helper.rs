use once_cell::sync::Lazy;

fn init() {
    dotenvy::dotenv().ok();

    let username = std::env::var("APP_ARCGIS_USERNAME")
        .expect("No username found; Missing env var: APP_ARCGIS_USERNAME");
    let password = std::env::var("APP_ARCGIS_PASSWORD")
        .expect("No password found; Missing env var: APP_ARCGIS_PASSWORD");
    let portal = std::env::var("APP_ARCGIS_PORTAL")
        .expect("No portal found; Missing env var: APP_ARCGIS_PORTAL");
    let referer = "127.0.0.1".to_string();
    let expiration = "1";

    let client = arcgis_sharing_rs::ArcGISSharingClient::builder()
        .portal(portal)
        .legacy_auth(username, password, referer, expiration)
        .build();
    arcgis_sharing_rs::initialise(client);
}

#[allow(dead_code)]
pub static SETUP: Lazy<()> = Lazy::new(|| {
    init();
});
