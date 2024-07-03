use serde::{Deserialize, Serialize};

use super::shared::FeedInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceFeed {
    #[serde(rename = "type")]
    pub geojson_type: String,
    pub feed_info: FeedInfo,
    pub features: Vec<FieldDeviceFeature>,
    pub bbox: Option<Vec<f64>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDeviceFeature {

}