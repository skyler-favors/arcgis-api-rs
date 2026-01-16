#![recursion_limit = "256"]
mod common;
use common::*;

use once_cell::sync::Lazy;

#[serial_test::serial]
mod item_tests {
    use super::*;

    //     #[tokio::test]
    //     async fn test_publish_csv_item_with_helper() {
    //         Lazy::force(&SETUP);
    //         let client = arcgis_sharing_rs::instance();
    //
    //         let uuid = uuid::Uuid::new_v4().to_string();
    //         let title = format!("test-publish-csv-{}", uuid);
    //         let service_name = format!("TestPublishService_{}", uuid.replace("-", "_"));
    //
    //         let test_csv = r#"id,timestamp,status,temp_c,Longitude,Latitude
    // 1025,2024-05-20T15:36:00Z,inactive,16.7,-109.33245320824183,41.39076580522106
    // 1026,2024-05-20T16:51:00Z,maintenance,17.4,-109.47781997822837,41.67733099029833
    // 1027,2024-05-20T17:53:00Z,inactive,30.4,-109.83293803634402,41.66539036168230
    // 1028,2024-05-20T18:05:00Z,maintenance,23.1,-109.15503559233532,41.44373625216593"#;
    //
    //         // First add a CSV item
    //         let add_response = client
    //             .content(None::<String>)
    //             .add_item()
    //             .file(test_csv)
    //             .set_type("CSV")
    //             .title(&title)
    //             .filename(format!("{}.csv", uuid))
    //             .send()
    //             .await
    //             .unwrap();
    //
    //         assert!(add_response.success);
    //         println!("Added item with ID: {}", add_response.id);
    //
    //         // Then publish it using the helper method
    //         let publish_response = client
    //             .item(None::<String>, &add_response.id)
    //             .publish()
    //             .csv_with_coordinates(&service_name, "Latitude", "Longitude")
    //             .send()
    //             .await
    //             .unwrap();
    //
    //         assert!(!publish_response.services.is_empty());
    //         assert!(!publish_response.services[0].service_item_id.is_empty());
    //         assert!(!publish_response.services[0].job_id.is_empty());
    //         println!(
    //             "Published service with item ID: {} and job ID: {}",
    //             publish_response.services[0].service_item_id, publish_response.services[0].job_id
    //         );
    //     }

    #[tokio::test]
    async fn test_publish_csv_item_with_manual_parameters() {
        Lazy::force(&SETUP);
        let client = arcgis_sharing_rs::instance();

        let uuid = uuid::Uuid::new_v4().to_string();
        let title = format!("test-publish-csv-manual-{}", uuid);
        let filename = format!("{}.csv", uuid);
        let service_name = format!("TestPublishManual_{}", uuid.replace("-", "_"));

        let test_csv = r#"Longitude,Latitude
-109.33245320824183,41.39076580522106
-109.47781997822837,41.67733099029833"#;

        // First add a CSV item
        let add_response = client
            .content(None::<String>)
            .add_item()
            .file(test_csv)
            .set_type("CSV")
            .title(&title)
            .filename(&filename)
            .send()
            .await
            .unwrap();

        assert!(add_response.success);
        println!("Added item with ID: {}", add_response.id);

        // Then publish it using manual parameters
        // let publish_params = serde_json::json!({
        //     "type": "csv",
        //     "name": service_name,
        //     "locationType": "coordinates",
        //     "latitudeFieldName": "Latitude",
        //     "longitudeFieldName": "Longitude",
        //     "coordinateFieldType": "LatitudeAndLongitude",
        //     "sourceSR": {
        //         "wkid": 4326,
        //         "latestWkid": 4326
        //     },
        //     "targetSR": {
        //         "wkid": 102100,
        //         "latestWkid": 3857
        //     }
        // });

        let fields = serde_json::json!([
            {
                "name": "Longitude",
                "type": "esriFieldTypeDouble",
                "alias": "Longitude",
                "locationType": "longitude",
                "sqlType": "sqlTypeDouble"
            },
            {
                "name": "Latitude",
                "type": "esriFieldTypeDouble",
                "alias": "Latitude",
                "locationType": "latitude",
                "sqlType": "sqlTypeDouble"
            }
        ]);
        // Add additional fields as strings
        // if let Some(fields_array) = fields.as_array_mut() {
        //     for field_name in &additional_fields {
        //         fields_array.push(serde_json::json!({
        //             "name": field_name,
        //             "type": "esriFieldTypeString",
        //             "alias": field_name,
        //             "sqlType": "sqlTypeOther",
        //             "length": 256
        //         }));
        //     }
        // }

        // Build template attributes: include all fields with null values
        let attributes = serde_json::json!({
            "Longitude": null,
            "Latitude": null
        });
        // if let Some(attrs_obj) = attributes.as_object_mut() {
        //     for field_name in &additional_fields {
        //         attrs_obj.insert(field_name.clone(), serde_json::Value::Null);
        //     }
        // }

        let publish_params = serde_json::json!({
            "type": "csv",
            "name": service_name,
            "sourceUrl": "",
            "maxRecordCount": 1000,
            "targetSR": {
                "wkid": 102100,
                "latestWkid": 3857
            },
            "editorTrackingInfo": {
                "enableEditorTracking": false,
                "enableOwnershipAccessControl": false,
                "allowOthersToUpdate": true,
                "allowOthersToDelete": false
            },
            "locationType": "coordinates",
            "latitudeFieldName": "Latitude",
            "longitudeFieldName": "Longitude",
            "sourceSR": {
                "wkid": 4326,
                "latestWkid": 4326
            },
            "columnDelimiter": ",",
            "layerInfo": {
                "currentVersion": 11.3,
                "id": 0,
                "name": uuid,
                "type": "Feature Layer",
                "displayField": "",
                "description": "",
                "copyrightText": "",
                "defaultVisibility": true,
                "editFieldsInfo": null,
                "relationships": [],
                "isDataVersioned": false,
                "supportsRollbackOnFailureParameter": true,
                "supportsAdvancedQueries": true,
                "supportsValidateSQL": true,
                "supportsCalculate": true,
                "advancedQueryCapabilities": {
                    "supportsReturningQueryExtent": true,
                    "supportsStatistics": true,
                    "supportsDistinct": true,
                    "supportsPagination": true,
                    "supportsOrderBy": true,
                    "supportsQueryWithDistance": true
                },
                "geometryType": "esriGeometryPoint",
                "drawingInfo": {
                    "renderer": {
                        "type": "simple",
                        "symbol": {
                            "type": "esriSMS",
                            "style": "esriSMSCircle",
                            "color": [129, 140, 0, 255],
                            "size": 4,
                            "angle": 0,
                            "xoffset": 0,
                            "yoffset": 0,
                            "outline": {
                                "color": [0, 0, 0, 255],
                                "width": 1
                            }
                        },
                        "label": "",
                        "description": ""
                    }
                },
                "hasM": false,
                "hasZ": false,
                "allowGeometryUpdates": true,
                "hasAttachments": false,
                "htmlPopupType": "esriServerHTMLPopupTypeNone",
                "supportsApplyEditsWithGlobalIds": true,
                "objectIdField": "",
                "globalIdField": "",
                "typeIdField": "",
                "fields": fields,
                "types": [],
                "templates": [
                    {
                        "name": "New Feature",
                        "description": "",
                        "drawingTool": "esriFeatureEditToolPoint",
                        "prototype": {
                            "attributes": attributes
                        }
                    }
                ],
                "useStandardizedQueries": true,
                "enableZDefaults": false,
                "zDefault": 0,
                "supportedQueryFormats": "JSON",
                "hasStaticData": true,
                "maxRecordCount": 1000,
                "capabilities": "Query",
                "supportsCoordinatesQuantization": false,
                "supportsAttachmentsByUploadId": true
            },
            "coordinateFieldType": "LatitudeAndLongitude",
            "capabilities": "Query",
            "hasStaticData": true,
            "persistErrorRecordsForReview": true,
            "dateFieldsTimeReference": {
                "timeZone": "UTC"
            }
        });

        println!(
            "publish params: {}",
            serde_json::to_string_pretty(&publish_params).unwrap()
        );

        let publish_response = client
            .item(None::<String>, &add_response.id)
            .publish()
            .set_file_type("csv")
            .set_publish_parameters(publish_params)
            .send()
            .await
            .unwrap();

        assert!(!publish_response.services.is_empty());
        assert!(!publish_response.services[0].service_item_id.is_empty());
        assert!(!publish_response.services[0].job_id.is_empty());
        assert_eq!(publish_response.services[0].service_type, "Feature Service");
        println!(
            "Published service with item ID: {} and job ID: {}",
            publish_response.services[0].service_item_id, publish_response.services[0].job_id
        );
    }
}
