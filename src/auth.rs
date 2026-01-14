use secrecy::SecretString;

/// State used to authenticate to ArcGIS
// #[derive(Default, Debug)]
// pub enum Auth {
//     /// No state
//     #[default]
//     None,
//
//     /// OAuth 2.0 - User Authentication (Authorization Code Flow)
//     /// This method prompts a user to sign in with their ArcGIS credentials. Your app receives an access token once the user grants permission.
//     /// Permissions: The token inherits all privileges of the signed-in user. If the user is an Administrator, the token has administrative rights.
//     /// Use Case: Apps where users need to access their own private maps, edit data, or perform analysis using their own credits.
//     /// Duration: Short-lived (usually 30 minutes to 2 hours), but can be refreshed using a refresh_token.
//     OAuthUser(OAuthUser),
//
//     /// OAuth 2.0 - App Authentication (Client Credentials Flow)
//     /// This uses a client_id and client_secret registered in your ArcGIS portal to generate a token without user interaction.
//     /// Permissions: Permissions are limited to the privileges assigned to the Application Item in the portal. It cannot "impersonate" a user's private content unless that content is shared with the application.
//     /// Use Case: Server-to-server communication, scheduled scripts, or public apps that need to access location services (like routing) billed to the developer's account.
//     OAuthApp(OAuthApp),
//
//     /// API Keys are long-lived access tokens created and managed through the developer dashboard or portal content.
//     /// Permissions: Scoped/Granular. You explicitly define what the key can do (e.g., "Allow Basemaps," "Allow Routing," or "Access Item X"). It does not have broad "user" permissions.
//     /// Use Case: Static apps, IoT devices, or simple web maps where you don't want to implement a full OAuth login flow.
//     /// Duration: Up to 1 year.
//     /// Legacy Note: If you are using "Legacy API Keys" (created before June 2024), these are set to expire by May 2026. You should migrate to the new "API Key Credentials" items.
//     APIKey(APIKey),
//
//     /// This is Esri’s proprietary method that predates OAuth 2.0. It involves sending a username and password directly to the /sharing/rest/generateToken endpoint.
//     /// Permissions: Inherits all privileges of the user credentials provided.
//     /// Use Case: Legacy scripts or ArcGIS Enterprise environments where OAuth is not yet configured. It is generally discouraged for new development due to the security risk of handling raw passwords.
//     /// Duration: Variable, typically 60 minutes.
//     LegacyToken(LegacyToken),
// }

#[allow(dead_code)]
#[derive(Debug)]
pub struct OAuthUser;

#[allow(dead_code)]
#[derive(Debug)]
pub struct OAuthApp {
    client_id: SecretString,
    client_secret: SecretString,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct APIKey {
    key: SecretString,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LegacyToken {
    username: SecretString,
    password: SecretString,
}
