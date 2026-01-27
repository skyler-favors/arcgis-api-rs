use arcgis_sharing_rs::ArcGISSharingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let portal = std::env::var("ARCGIS_PORTAL")?;
    let username = std::env::var("ARCGIS_USERNAME")?;
    let password = std::env::var("ARCGIS_PASSWORD")?;
    let referer = std::env::var("ARCGIS_REFERER")?;

    // Create client
    let client = ArcGISSharingClient::builder()
        .portal(portal)
        .legacy_auth(username, password, referer, "60".to_string())
        .build();

    // Sample CSV data with coordinate fields
    let csv_data = r#"id,timestamp,status,temp_c,Longitude,Latitude
1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;

    // Step 1: Add the CSV item to the portal
    println!("Adding CSV item to portal...");
    let add_response = client
        .content(None::<String>)
        .add_item()
        .file(csv_data)
        .set_type("CSV")
        .title("Temperature Sensors")
        .send()
        .await?;

    println!("Item added with ID: {}", add_response.id);

    // Step 2: Analyze the CSV to get publish parameters
    println!("\nAnalyzing CSV for publish parameters...");
    let analyze_response = client
        .content(None::<String>)
        .analyze()
        .set_item_id(&add_response.id)
        .set_filetype("csv")
        .send()
        .await?;

    println!("Analysis complete!");
    println!(
        "Publish parameters: {}",
        serde_json::to_string_pretty(&analyze_response.publish_parameters)?
    );

    // Step 3: Publish the CSV as a feature service using the analyzed parameters
    println!("\nPublishing CSV as feature service...");
    let publish_response = client
        .item(None::<String>, &add_response.id)
        .publish()
        .set_file_type("csv")
        .set_publish_parameters(analyze_response.publish_parameters)
        .send()
        .await?;

    println!("Successfully published feature service!");
    for service in &publish_response.services {
        println!("  Service URL: {}", service.serviceurl);
        println!("  Service Type: {}", service.service_type);
        println!("  Service Item ID: {}", service.service_item_id);
        println!("  Job ID: {}", service.job_id);
    }

    Ok(())
}
