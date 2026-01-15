mod community;
mod feature_service;

pub use community::*;
pub use feature_service::*;
use serde::Serializer;

/// Serializes a Vec<String> or Vec<T: Display> as a single comma-separated string.
fn serialize_comma_separated<S, T>(vec: &[T], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    let combined = vec
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",");
    s.serialize_str(&combined)
}

/// Custom serializer to convert Vec<serde_json::Value> to JSON string
/// ArcGIS REST API expects arrays to be serialized as JSON strings
fn serialize_json_string<S>(
    value: &Vec<serde_json::Value>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_string = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}
