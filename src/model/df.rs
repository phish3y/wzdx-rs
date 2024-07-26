use geo::Geometry;
use serde::{Deserialize, Serialize};

use super::shared::FeedInfo;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeviceFeed {
    pub r#type: String,
    pub feed_info: FeedInfo,
    pub features: Vec<FieldDeviceFeature>,
    pub bbox: Option<Vec<f64>>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FieldDeviceFeature {
    pub id: String,
    pub r#type: String,
    pub properties: Properties,
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    pub geometry: Geometry,
    pub bbox: Option<Vec<f64>>
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
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
    pub location_marker: Option<LocationMarker>,
    #[serde(flatten)]
    pub traffic_sensor: Option<TrafficSensor>,
    #[serde(flatten)]
    pub traffic_signal: Option<TrafficSignal>
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ArrowBoard {
    pub pattern: String,
    #[deprecated]
    pub is_moving: Option<bool>,
    pub is_in_transport_position: Option<bool>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Camera {
    pub image_url: Option<String>,
    pub image_timestamp: Option<String>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DynamicMessageSign {
    pub message_multi_string: String
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FlashingBeacon {
    pub function: String,
    pub is_flashing: Option<bool>,
    pub sign_text: Option<String>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HybridSign {
    pub dynamic_message_function: String,
    pub dynamic_message_text: Option<String>,
    pub static_sign_text: Option<String>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LocationMarker {
    pub marked_locations: Vec<MarkedLocationType>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MarkedLocationType {
    pub r#type: String,
    pub road_event_id: Option<String>
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TrafficSensor {
    pub collection_interval_start_date: String,
    pub collection_interval_end_date: String,
    pub average_speed_kph: Option<f64>,
    pub volume_vph: Option<f64>,
    pub occupancy_percent: Option<f64>,
    pub lane_data: Option<Vec<TrafficSensorLaneData>>
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TrafficSensorLaneData {
    pub lane_order: u64,
    pub road_event_id: Option<String>,
    pub average_speed_kph: Option<f64>,
    pub volume_vph: Option<f64>,
    pub occupancy_percent: Option<f64>
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TrafficSignal {
    pub mode: String
}