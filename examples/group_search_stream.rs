use arcgis_sharing_rs::ArcGISSharingClient;
use futures::StreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = ArcGISSharingClient::builder()
        .portal("https://www.arcgis.com".to_string())
        .build();

    println!("Searching for 'water' related groups...\n");

    // Create a group search stream
    // Note: By default, there's a 100ms delay between page fetches
    let mut search_stream = client
        .search_groups()
        .query("water")
        .set_num(10) // 10 groups per page
        .set_max_pages(3) // Limit to 3 pages (30 groups max)
        .set_page_fetch_delay(Duration::from_millis(100)) // Optional: customize delay
        .send();

    // Iterate through results one at a time
    let mut count = 0;
    while let Some(group) = search_stream.next().await {
        count += 1;
        println!(
            "{:>3}. {} (owner: {}, access: {})",
            count, group.title, group.owner, group.access
        );

        if count >= 20 {
            println!("\n(Stopping after 20 groups...)");
            break;
        }
    }

    println!("\nTotal groups fetched: {}", count);

    // Example: Collect all results into a Vec
    println!("\n--- Using collect() ---");
    let results: Vec<_> = client
        .search_groups()
        .query("environment")
        .set_num(5)
        .set_max_pages(2)
        .send()
        .collect()
        .await;

    println!("Collected {} environment groups", results.len());

    // Example: Using take() to limit results
    println!("\n--- Using take() ---");
    let limited_results: Vec<_> = client
        .search_groups()
        .query("access:public")
        .set_num(20)
        .send()
        .take(5) // Only take first 5 groups
        .collect()
        .await;

    println!("Took {} public groups", limited_results.len());

    // Example: Disable delay for faster fetching
    println!("\n--- Fast fetching (no delay) ---");
    let fast_results: Vec<_> = client
        .search_groups()
        .query("GIS")
        .set_num(10)
        .set_max_pages(2)
        .set_page_fetch_delay(Duration::ZERO) // No delay between pages
        .send()
        .collect()
        .await;

    println!(
        "Quickly fetched {} GIS groups (no delay)",
        fast_results.len()
    );

    // Example: Using filter for exact matches
    println!("\n--- Using filter ---");
    let filtered_stream = client
        .search_groups()
        .filter("title:\"Water Resources\"")
        .set_num(10)
        .send();

    let filtered_results: Vec<_> = filtered_stream.collect().await;
    println!("Found {} groups with exact title match", filtered_results.len());

    // Example: Sort by creation date
    println!("\n--- Sorted by creation date (newest first) ---");
    let sorted_results: Vec<_> = client
        .search_groups()
        .query("climate")
        .set_sort_field("created")
        .set_sort_order("desc")
        .set_num(5)
        .send()
        .collect()
        .await;

    println!("Found {} climate groups (sorted by date):", sorted_results.len());
    for group in sorted_results {
        println!("  - {} (created: {})", group.title, group.created);
    }

    Ok(())
}
