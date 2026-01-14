use std::fmt;

use secrecy::SecretString;

use crate::auth::Auth;

#[derive(Debug, Clone)]
pub enum AuthState {
    None,

    OAuthUser,

    OAuthApp {
        client_id: SecretString,
        client_secret: SecretString,
    },

    APIKey {
        key: SecretString,
    },

    LegacyToken {
        username: SecretString,
        password: SecretString,
    },
}

#[derive(Clone)]
pub struct ArcGISSharingClient {
    client: reqwest::Client,
    auth_state: AuthState,
}

impl fmt::Debug for ArcGISSharingClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcGISSharingClient")
            .field("auth_state", &self.auth_state)
            .finish()
    }
}

impl Default for ArcGISSharingClient {
    fn default() -> ArcGISSharingClient {
        ArcGISSharingClient {
            client: reqwest::Client::new(),
            auth_state: AuthState::None,
        }
    }
}

/// # ArcGIS Sharing API Methods
impl ArcGISSharingClient {
    // pub fn actions(&self) -> actions::ActionsHandler<'_> {
    //     actions::ActionsHandler::new(self)
    // }
}

#[derive(Default)]
pub struct ArcGISSharingClientBuilder {
    auth: Auth,
}

impl ArcGISSharingClientBuilder {
    pub fn new() -> Self {
        ArcGISSharingClientBuilder::default()
    }

    pub fn build() -> ArcGISSharingClient {
        ArcGISSharingClient {
            client: (),
            auth_state: (),
        }
    }
}
