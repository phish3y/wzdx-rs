use geo::Geometry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkZoneFeed {
    #[serde(rename = "type")]
    pub geojson_type: String,
    pub features: Vec<RoadEventFeature>,
    pub feed_info: Option<FeedInfo>,
    pub road_event_feed_info: Option<FeedInfo>,
    pub bbox: Option<Vec<f64>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoadEventFeature {
    pub id: String,
    #[serde(rename = "type")]
    pub geojson_object_type: String,
    pub properties: Properties,
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    pub geometry: Geometry
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    pub core_details: RoadEventCoreDetails,
    #[serde(flatten)]
    pub work_zone: Option<WorkZoneRoadEvent>,
    #[serde(flatten)]
    pub detour: Option<DetourRoadEvent>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoadEventCoreDetails {
    pub event_type: String,
    pub data_source_id: String,
    pub direction: String,
    pub road_names: Vec<String>,
    pub related_road_events: Option<Vec<RelatedRoadEvent>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub update_date: Option<String>,
    pub relationship: Option<Relationship> // DEPRECATED
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedRoadEvent {
    #[serde(rename = "type")]
    pub road_event_type: String,
    pub id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Relationship {
    pub first: Option<Vec<String>>,
    pub next: Option<Vec<String>>,
    pub parents: Option<Vec<String>>,
    pub children: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkZoneRoadEvent {
    pub start_date: String,
    pub end_date: String,
    pub vehicle_impact: String,
    pub location_method: String,
    pub beginning_cross_street: Option<String>,
    pub ending_cross_street: Option<String>,
    pub beginning_milepost: Option<f64>,
    pub ending_milepost: Option<f64>,
    pub is_start_position_verified: Option<bool>,
    pub is_end_position_verified: Option<bool>,
    pub is_start_date_verified: Option<bool>,
    pub is_end_date_verified: Option<bool>,
    pub work_zone_type: Option<String>,
    pub worker_presence: Option<WorkerPresence>,
    pub reduced_speed_limit_kph: Option<f64>,
    pub restrictions: Option<Vec<Restriction>>,
    pub types_of_work: Option<Vec<TypeOfWOrk>>,
    pub lanes: Option<Vec<Lane>>,
    pub impacted_cds_curb_zones: Option<Vec<CdsCurbZonesReference>>,
    pub event_status: Option<String>, // DEPRECATED
    pub start_date_accuracy: Option<String>, // DEPRECATED
    pub end_date_accuracy: Option<String>, // DEPRECATED
    pub beginning_accuracy: Option<String>, // DEPRECATED
    pub ending_accuracy: Option<String> // DEPRECATED
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkerPresence {
    pub are_workers_present: bool,
    pub method: Option<String>,
    pub worker_presence_last_confirmed_date: Option<String>,
    pub confidence: Option<String>,
    pub definition: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Restriction {
    #[serde(rename = "type")]
    pub restriction_type: String,
    pub option: Option<f64>,
    pub unit: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeOfWOrk {
    pub type_name: String,
    pub is_architectural_change: Option<bool>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lane {
    pub status: String,
    #[serde(rename = "type")]
    pub lane_type: String,
    pub order: u64,
    pub lane_number: Option<u64>,
    pub restrictions: Option<Vec<Restriction>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CdsCurbZonesReference {
    pub cds_curb_zone_ids: Vec<String>,
    pub cds_curbs_api_url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DetourRoadEvent {
    pub start_date: String,
    pub end_date: String,
    pub beginning_cross_street: Option<String>,
    pub ending_cross_street: Option<String>,
    pub beginning_milepost: Option<f64>,
    pub ending_milepost: Option<f64>,
    pub is_start_date_verified: Option<bool>,
    pub is_end_date_verified: Option<bool>,
    pub event_status: Option<String>, // DEPRECATED
    pub start_date_accuracy: Option<String>, // DEPRECATED
    pub end_date_accuracy: Option<String> // DEPRECATED
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedInfo {
    pub update_date: String,
    pub version: String,
    pub publisher: String,
    pub data_sources: Vec<FeedDataSource>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub update_frequency: Option<u64>,
    pub license: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedDataSource {
    pub data_source_id: String,
    pub organization_name: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub update_frequency: Option<u64>,
    pub update_date: Option<String>,
    pub lrs_type: Option<String>, // DEPRECATED
    pub lrs_url: Option<String>, // DEPRECATED
    pub location_verify_method: Option<String> // DEPRECATED
}