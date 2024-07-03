use geo::Geometry;
use serde::{Deserialize, Serialize};

use super::shared::FeedInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceFeed {
    pub r#type: String,
    pub feed_info: FeedInfo,
    pub features: Vec<FieldDeviceFeature>,
    pub bbox: Option<Vec<f64>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDeviceFeature {
    pub id: String,
    pub r#type: String,
    pub properties: Properties,
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    pub geometry: Geometry,
    pub bbox: Option<Vec<f64>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    pub core_details: FieldDeviceCoreDetails,
    #[serde(flatten)]
    pub arrow_board: Option<ArrowBoard>,
    #[serde(flatten)]
    pub camera: Option<Camera>,
    #[serde(flatten)]
    pub dynamic_message_sign: Option<DynamicMessageSign>,
    #[serde(flatten)]
    pub flashing_becon: Option<FlashingBeacon>,
    #[serde(flatten)]
    pub hybrid_sign: Option<HybridSign>,
    #[serde(flatten)]
    pub location_marker: Option<LocationMarker>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDeviceCoreDetails {
    pub device_type: String,
    pub data_source_id: String,
    pub device_status: String,
    pub update_date: String,
    pub has_automatic_location: bool,
    pub road_direction: Option<String>,
    pub road_names: Option<Vec<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status_message: Option<Vec<String>>,
    pub is_moving: Option<bool>,
    pub road_event_ids: Option<Vec<String>>,
    pub milepost: Option<f64>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub velocity_kph: Option<f64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArrowBoard {
    pub pattern: String,
    pub is_moving: Option<bool>, // DEPRECATED
    pub is_in_transport_position: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub image_url: Option<String>,
    pub image_timestamp: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicMessageSign {
    pub message_multi_string: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlashingBeacon {
    pub function: String,
    pub is_flashing: Option<bool>,
    pub sign_text: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HybridSign {
    pub dynamic_message_function: String,
    pub dynamic_message_text: Option<String>,
    pub static_sign_text: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationMarker {
    pub marked_locations: Vec<MarkedLocationType>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarkedLocationType {
    pub r#type: String,
    pub road_event_id: Option<String>
}
