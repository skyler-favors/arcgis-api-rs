use arcgis_sharing_rs::{
    auth::{
        exchange_oauth_authorization_code_with_client,
        exchange_oauth_refresh_token_credential_with_client,
        exchange_oauth_refresh_token_with_client,
    },
    error::{Error, OAuthError},
};
use std::io::{Read, Write};
use std::sync::mpsc;

const AUTHORIZATION_CODE: &str = include_str!("fixtures/oauth/authorization_code.json");
const REFRESH_TOKEN: &str = include_str!("fixtures/oauth/refresh_token.json");
const EXCHANGE_REFRESH_TOKEN: &str = include_str!("fixtures/oauth/exchange_refresh_token.json");
const INVALID_GRANT: &str = include_str!("fixtures/oauth/invalid_grant.json");
const RATE_LIMITED: &str = include_str!("fixtures/oauth/rate_limited.json");
const SERVER_ERROR: &str = include_str!("fixtures/oauth/server_error.json");
const CLIENT_ERROR: &str = include_str!("fixtures/oauth/client_error.json");
const MALFORMED: &str = include_str!("fixtures/oauth/malformed.json");

fn oauth_error(error: Error) -> Box<OAuthError> {
    match error {
        Error::OAuth { source, .. } => source,
        other => panic!("expected OAuth error, got {other}"),
    }
}

fn mock_response(status: u16, body: &str) -> (String, mpsc::Receiver<String>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind fixture server");
    let address = listener.local_addr().expect("fixture server address");
    let body = body.to_owned();
    let (request_tx, request_rx) = mpsc::channel();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept fixture request");
        let mut request = Vec::new();
        let mut buffer = [0; 4096];
        loop {
            let bytes_read = stream.read(&mut buffer).expect("read fixture request");
            request.extend_from_slice(&buffer[..bytes_read]);
            let Some(headers_end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n")
            else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..headers_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .map(str::to_owned)
                })
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(0);
            if request.len() >= headers_end + 4 + content_length {
                break;
            }
        }
        request_tx
            .send(String::from_utf8_lossy(&request).into_owned())
            .expect("send captured request");
        write!(
            stream,
            "HTTP/1.1 {status} Fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .expect("write fixture response");
    });
    (format!("http://{address}/token"), request_rx)
}

#[tokio::test]
async fn authorization_code_response_retains_refresh_expiration() {
    let (url, request) = mock_response(200, AUTHORIZATION_CODE);

    let response = exchange_oauth_authorization_code_with_client(
        &reqwest::Client::new(),
        &url,
        "client",
        "code",
        "https://app.example/callback",
        "verifier",
    )
    .await
    .expect("authorization code response");

    assert_eq!(response.refresh_token, "refresh-token");
    assert_eq!(response.refresh_token_expires_in, 1_209_600);
    assert!(request
        .recv()
        .expect("captured request")
        .contains("grant_type=authorization_code"));
}

#[tokio::test]
async fn refresh_response_only_models_a_new_access_token() {
    let (url, _request) = mock_response(200, REFRESH_TOKEN);
    let response = exchange_oauth_refresh_token_with_client(
        &reqwest::Client::new(),
        &url,
        "client",
        "refresh-token",
    )
    .await
    .expect("refresh response");

    assert_eq!(response.access_token, "refreshed-access-token");
}

#[tokio::test]
async fn exchange_refresh_token_sends_redirect_and_returns_replacement() {
    let (url, request) = mock_response(200, EXCHANGE_REFRESH_TOKEN);

    let response = exchange_oauth_refresh_token_credential_with_client(
        &reqwest::Client::new(),
        &url,
        "client",
        "https://app.example/callback",
        "refresh-token",
    )
    .await
    .expect("refresh token exchange response");

    assert_eq!(response.refresh_token, "replacement-refresh-token");
    assert_eq!(response.refresh_token_expires_in, 1_209_600);
    let request = request.recv().expect("captured request");
    assert!(request.contains("grant_type=exchange_refresh_token"));
    assert!(request.contains("redirect_uri=https%3A%2F%2Fapp.example%2Fcallback"));
}

#[tokio::test]
async fn classifies_explicit_invalid_refresh_credential() {
    let (url, _request) = mock_response(400, INVALID_GRANT);
    let error = exchange_oauth_refresh_token_with_client(
        &reqwest::Client::new(),
        &url,
        "client",
        "expired",
    )
    .await
    .expect_err("invalid refresh credential");

    assert!(matches!(
        *oauth_error(error),
        OAuthError::InvalidRefreshCredential { .. }
    ));
}

#[tokio::test]
async fn classifies_rate_limit_server_client_and_malformed_responses() {
    for (status, body, expected) in [
        (429, RATE_LIMITED, "rate"),
        (503, SERVER_ERROR, "server"),
        (403, CLIENT_ERROR, "client"),
        (200, MALFORMED, "malformed"),
    ] {
        let (url, _request) = mock_response(status, body);
        let error = exchange_oauth_refresh_token_with_client(
            &reqwest::Client::new(),
            &url,
            "client",
            "refresh-token",
        )
        .await
        .expect_err("classified failure");
        let error = oauth_error(error);
        assert!(match expected {
            "rate" => matches!(*error, OAuthError::RateLimited { .. }),
            "server" => matches!(*error, OAuthError::Server { .. }),
            "client" => matches!(*error, OAuthError::Client { .. }),
            "malformed" => matches!(*error, OAuthError::MalformedResponse { .. }),
            _ => unreachable!(),
        });
    }
}

#[tokio::test]
async fn classifies_transport_failures() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("reserve port");
    let address = listener.local_addr().expect("reserved address");
    drop(listener);

    let error = exchange_oauth_refresh_token_with_client(
        &reqwest::Client::new(),
        &format!("http://{address}/token"),
        "client",
        "refresh-token",
    )
    .await
    .expect_err("transport failure");

    assert!(matches!(*oauth_error(error), OAuthError::Transport { .. }));
}
