use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;

use arcgis_sharing_rs::ArcGISSharingClient;
use serde_json::json;

#[test]
fn serializes_definition_as_json() {
    let client = ArcGISSharingClient::builder()
        .portal("https://example.com".to_string())
        .admin_url("https://services.example.com/rest/admin/")
        .build();
    let handler = client.admin().feature_service("Test");
    let builder = handler.add_to_definition(json!({
        "layers": [{ "name": "Test layer" }]
    }));

    let serialized = serde_urlencoded::to_string(&builder).unwrap();
    let fields: HashMap<_, _> = url::form_urlencoded::parse(serialized.as_bytes())
        .into_owned()
        .collect();
    let definition: serde_json::Value = serde_json::from_str(&fields["addToDefinition"]).unwrap();

    assert_eq!(definition["layers"][0]["name"], "Test layer");
}

#[test]
#[should_panic(expected = "No admin URL provided")]
fn admin_handler_requires_configured_url() {
    ArcGISSharingClient::default().admin();
}

#[tokio::test]
async fn derives_feature_service_admin_url() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0; 4096];
        let bytes_read = stream.read(&mut request).unwrap();
        let request = String::from_utf8_lossy(&request[..bytes_read]);
        sender
            .send(request.lines().next().unwrap().to_string())
            .unwrap();

        let body = r#"{"success":true}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
        .unwrap();
    });

    let client = ArcGISSharingClient::builder()
        .portal("https://example.com".to_string())
        .admin_url(format!("http://{address}/rest/admin"))
        .build();

    let response = client
        .admin()
        .feature_service("Folder/Test Service")
        .add_to_definition(json!({ "layers": [] }))
        .send()
        .await
        .unwrap();

    assert!(response.success);
    assert!(receiver.recv().unwrap().starts_with(
        "POST /rest/admin/services/Folder/Test%20Service/FeatureServer/addToDefinition?"
    ));
}
