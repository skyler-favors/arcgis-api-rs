mod common;
use common::*;

use futures::StreamExt;
use once_cell::sync::Lazy;

#[serial_test::serial]
mod search_tests {
    use super::*;

    #[tokio::test]
    async fn test_search_stream_basic() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let mut search_stream = client
            .search()
            .query("water") // Simple query that should match many items
            .set_num(5) // Small page size to test pagination
            .send();

        let mut count = 0;
        while let Some(result) = search_stream.next().await {
            count += 1;

            // Verify result has expected fields
            assert!(!result.id.is_empty());
            assert!(!result.title.is_empty());
            assert!(!result.owner.is_empty());

            // Stop after a few items to keep test fast
            if count >= 10 {
                break;
            }
        }

        assert!(count > 0, "Should have fetched at least one result");
    }

    #[tokio::test]
    async fn test_search_stream_pagination() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Use small page size to ensure multiple pages are fetched
        let mut search_stream = client
            .search()
            .query("type:\"Web Map\" AND owner:esri_livingatlas")
            .set_num(3) // Very small page size
            .set_max_pages(3) // Limit to 3 pages
            .send();

        let mut count = 0;
        while let Some(result) = search_stream.next().await {
            count += 1;
            println!("Item {}: {}", count, result.title);
        }

        println!("Total items with pagination: {}", count);
        // Should fetch up to 9 items (3 pages * 3 items per page)
        assert!(count > 3, "Should have fetched more than one page");
        assert!(count <= 9, "Should respect max_pages limit");
    }

    #[tokio::test]
    async fn test_search_stream_max_pages_limit() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Limit to just 2 pages
        let mut search_stream = client
            .search()
            .query("owner:esri")
            .set_num(5)
            .set_max_pages(2)
            .send();

        let mut count = 0;
        while let Some(_result) = search_stream.next().await {
            count += 1;
        }

        println!("Total items with max_pages=2: {}", count);
        // Should fetch at most 10 items (2 pages * 5 items per page)
        assert!(count <= 10, "Should respect max_pages limit of 2");
    }

    #[tokio::test]
    async fn test_search_stream_with_filters() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let mut search_stream = client
            .search()
            .query("water")
            .filter("type:\"Feature Service\"")
            .set_num(5)
            .set_max_pages(2)
            .send();

        let mut count = 0;
        while let Some(result) = search_stream.next().await {
            count += 1;
            println!("Filtered result: {} - {}", result.title, result.type_field);
        }

        println!("Total filtered items: {}", count);
    }

    #[tokio::test]
    async fn test_search_stream_collect() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Test using collect to gather all results
        let results: Vec<_> = client
            .search()
            .query("owner:esri_livingatlas AND type:\"Web Map\"")
            .set_num(3)
            .set_max_pages(2)
            .send()
            .collect()
            .await;

        println!("Collected {} results", results.len());
        assert!(results.len() > 0, "Should have collected some results");
        assert!(
            results.len() <= 6,
            "Should respect max_pages when collecting"
        );
    }

    #[tokio::test]
    async fn test_search_stream_take() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Test using take to limit results
        let results: Vec<_> = client
            .search()
            .query("type:\"Feature Service\"")
            .set_num(10)
            .send()
            .take(5) // Only take first 5 items
            .collect()
            .await;

        println!("Took {} results", results.len());
        assert_eq!(results.len(), 5, "Should have taken exactly 5 results");
    }

    #[tokio::test]
    async fn test_search_stream_enriched() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Test with enriched parameter
        let mut search_stream = client
            .search()
            .query("basemap")
            .set_enriched(true)
            .set_num(3)
            .set_max_pages(1)
            .send();

        let mut count = 0;
        while let Some(result) = search_stream.next().await {
            count += 1;
            println!("Enriched result: {}", result.title);
        }

        println!("Total enriched results: {}", count);
    }

    #[tokio::test]
    async fn test_search_stream_empty_results() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Search for something that likely returns no results
        let mut search_stream = client
            .search()
            .query("xyzabc123nonexistentquery999")
            .set_num(10)
            .send();

        let mut count = 0;
        while let Some(_result) = search_stream.next().await {
            count += 1;
        }

        println!("Empty search returned {} results", count);
        // This might return 0 or a few results depending on the portal
        assert!(
            count < 5,
            "Should return very few or no results for nonsense query"
        );
    }

    #[tokio::test]
    async fn test_search_stream_no_delay() {
        use std::time::{Duration, Instant};

        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Test with no delay - should be faster
        let start = Instant::now();
        let results: Vec<_> = client
            .search()
            .query("water")
            .set_num(3)
            .set_max_pages(3)
            .set_page_fetch_delay(Duration::ZERO) // No delay
            .send()
            .collect()
            .await;

        let duration = start.elapsed();

        println!(
            "Fetched {} results in {:?} with no delay",
            results.len(),
            duration
        );

        // With no delay and 3 pages, should complete faster than 1 second
        // (assuming each API call takes < 333ms on average)
        assert!(results.len() > 0, "Should have fetched some results");
        println!("Test completed in {:?}", duration);
    }

    #[tokio::test]
    async fn test_search_stream_custom_delay() {
        use std::time::{Duration, Instant};

        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        // Test with custom delay
        let start = Instant::now();
        let results: Vec<_> = client
            .search()
            .query("water")
            .set_num(3)
            .set_max_pages(2)
            .set_page_fetch_delay(Duration::from_millis(100)) // 100ms delay
            .send()
            .collect()
            .await;

        let duration = start.elapsed();

        println!(
            "Fetched {} results in {:?} with 100ms delay",
            results.len(),
            duration
        );

        // With 2 pages, we should have at least 1 delay (between page 1 and 2)
        // So total time should be at least 100ms
        assert!(results.len() > 0, "Should have fetched some results");
        assert!(
            duration >= Duration::from_millis(100),
            "Should have taken at least 100ms with delay"
        );
    }

    #[tokio::test]
    async fn test_living_atlas_search() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let results: Vec<_> = client
            .search()
            //.query("orgid:0123456789ABCDEF AND (type:\"Feature Service\" (water))")
            .query("owner:esri_livingatlas AND (type:\"Feature Service\" (water))")
            .set_num(5)
            .set_max_pages(1)
            .send()
            .collect()
            .await;

        // println!("Found {} results", results.len());
        // for result in results {
        //     println!("{} {} {}", result.title, result.type_field, result.owner);
        // }

        assert!(results.len() > 0, "Should have found some results");
    }
}
