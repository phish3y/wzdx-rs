use geo::Geometry;
use serde::{Deserialize, Serialize};

use super::shared::FeedInfo;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkZoneFeed {
    pub r#type: String,
    pub feed_info: Option<FeedInfo>,
    pub features: Vec<RoadEventFeature>,
    pub road_event_feed_info: Option<FeedInfo>,
    pub bbox: Option<Vec<f64>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoadEventFeature {
    pub id: String,
    pub r#type: String,
    pub properties: Properties,
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    pub geometry: Geometry
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    pub core_details: RoadEventCoreDetails,
    #[serde(flatten)]
    pub work_zone: Option<WorkZoneRoadEvent>,
    #[serde(flatten)]
    pub detour: Option<DetourRoadEvent>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[deprecated]
    pub relationship: Option<Relationship>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelatedRoadEvent {
    pub r#type: String,
    pub id: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Relationship {
    pub first: Option<Vec<String>>,
    pub next: Option<Vec<String>>,
    pub parents: Option<Vec<String>>,
    pub children: Option<Vec<String>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[deprecated]
    pub event_status: Option<String>,
    #[deprecated]
    pub start_date_accuracy: Option<String>,
    #[deprecated]
    pub end_date_accuracy: Option<String>,
    #[deprecated]
    pub beginning_accuracy: Option<String>,
    #[deprecated]
    pub ending_accuracy: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkerPresence {
    pub are_workers_present: bool,
    pub method: Option<String>,
    pub worker_presence_last_confirmed_date: Option<String>,
    pub confidence: Option<String>,
    pub definition: Option<Vec<String>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restriction {
    pub r#type: String,
    pub option: Option<f64>,
    pub unit: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeOfWOrk {
    pub type_name: String,
    pub is_architectural_change: Option<bool>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lane {
    pub status: String,
    pub r#type: String,
    pub order: u64,
    pub lane_number: Option<u64>,
    pub restrictions: Option<Vec<Restriction>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CdsCurbZonesReference {
    pub cds_curb_zone_ids: Vec<String>,
    pub cds_curbs_api_url: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DetourRoadEvent {
    pub start_date: String,
    pub end_date: String,
    pub beginning_cross_street: Option<String>,
    pub ending_cross_street: Option<String>,
    pub beginning_milepost: Option<f64>,
    pub ending_milepost: Option<f64>,
    pub is_start_date_verified: Option<bool>,
    pub is_end_date_verified: Option<bool>,
    #[deprecated]
    pub event_status: Option<String>,
    #[deprecated]
    pub start_date_accuracy: Option<String>,
    #[deprecated]
    pub end_date_accuracy: Option<String>
}
