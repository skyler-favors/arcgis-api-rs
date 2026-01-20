mod common;
use common::*;

use futures::StreamExt;
use once_cell::sync::Lazy;

#[serial_test::serial]
mod search_tests {
    use super::*;

    #[tokio::test]
    async fn test_group_search_basic() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let results: Vec<_> = client
            .search_groups()
            .query("title:test")
            .set_num(5)
            .set_max_pages(5)
            .send()
            .collect()
            .await;

        // for result in results.iter() {
        //     println!("{}: {}", result.title, result.owner);
        // }

        assert!(results.len() > 0, "Should have fetched at least one result");
    }
}
