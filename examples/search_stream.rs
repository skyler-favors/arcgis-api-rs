use arcgis_sharing_rs::ArcGISSharingClient;
use futures::StreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = ArcGISSharingClient::builder()
        .portal("https://www.arcgis.com".to_string())
        .build();

    println!("Searching for 'water' related items...\n");

    // Create a search stream
    // Note: By default, there's a 500ms delay between page fetches
    let mut search_stream = client
        .search()
        .query("water")
        .set_num(10) // 10 items per page
        .set_max_pages(3) // Limit to 3 pages (30 items max)
        .set_page_fetch_delay(Duration::from_millis(500)) // Optional: customize delay
        .send();

    // Iterate through results one at a time
    let mut count = 0;
    while let Some(result) = search_stream.next().await {
        count += 1;
        println!("{:>3}. {} ({})", count, result.title, result.owner);
        
        if count >= 20 {
            println!("\n(Stopping after 20 items...)");
            break;
        }
    }

    println!("\nTotal items fetched: {}", count);

    // Example: Collect all results into a Vec
    println!("\n--- Using collect() ---");
    let results: Vec<_> = client
        .search()
        .query("basemap")
        .set_num(5)
        .set_max_pages(2)
        .send()
        .collect()
        .await;

    println!("Collected {} basemap items", results.len());

    // Example: Using take() to limit results
    println!("\n--- Using take() ---");
    let limited_results: Vec<_> = client
        .search()
        .query("type:\"Feature Service\"")
        .set_num(20)
        .send()
        .take(5) // Only take first 5 items
        .collect()
        .await;

    println!("Took {} feature service items", limited_results.len());

    // Example: Disable delay for faster fetching
    println!("\n--- Fast fetching (no delay) ---");
    let fast_results: Vec<_> = client
        .search()
        .query("basemap")
        .set_num(10)
        .set_max_pages(2)
        .set_page_fetch_delay(Duration::ZERO) // No delay between pages
        .send()
        .collect()
        .await;

    println!(
        "Quickly fetched {} basemap items (no delay)",
        fast_results.len()
    );

    Ok(())
}
