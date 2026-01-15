mod common;
use common::*;

use arcgis_sharing_rs::ArcGISSharingClient;
use once_cell::sync::Lazy;

#[serial_test::serial]
mod community_tests {
    use super::*;

    async fn create_group(client: &ArcGISSharingClient) -> String {
        // create test group name
        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("test-{}", uuid);

        let create_result = client
            .create_group()
            .create(&title)
            .tags(vec!["test", "dev"])
            .send()
            .await
            .expect("Failed to send create group query");

        let group = create_result.group;
        assert!(&group.title == &title);
        return group.id;
    }

    async fn delete_group(client: &ArcGISSharingClient, group_id: &str) {
        let delete_result = client.groups(group_id).delete().send().await.unwrap();
        assert!(delete_result.success);
        assert!(delete_result.group_id == group_id);
    }

    #[tokio::test]
    async fn test_group_lifecycle() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();
        let group_id = create_group(&client).await;
        delete_group(&client, &group_id).await;
    }
}
