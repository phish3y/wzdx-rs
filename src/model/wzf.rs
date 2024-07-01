use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkZoneFeed {
    #[serde(rename = "type")]
    pub geojson_type: String, // TODO enum constrained
    pub features: Vec<RoadEventFeature>,
    pub feed_info: Option<FeedInfo>, // TODO this or road_event_feed_info
    // TODO
}

#[derive(Debug, Deserialize)]
pub struct RoadEventFeature {
    pub id: String,
    #[serde(rename = "type")]
    pub geojson_object_type: String, // TODO enum constrained
    pub properties: Properties,
    pub goemetry: Geometry
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub core_details: RoadEventCoreDetails,
    // TODO
}

#[derive(Debug, Deserialize)]
pub struct RoadEventCoreDetails {
    pub event_type: String, // TODO enum constrained
    pub data_source_id: String,
    pub direction: String, // TODO enum constrained
    pub road_names: Vec<String>, // TODO min 1
    pub related_road_events: Option<Vec<RelatedRoadEvent>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>, // TODO validate
    pub update_date: Option<String>, // TODO validate
    pub relationship: Option<Relationship> // DEPRECATED

}

#[derive(Debug, Deserialize)]
pub struct RelatedRoadEvent {
    pub road_event_type: String, // TODO enum constrained
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
    pub first: Option<Vec<String>>, // TODO min 1
    pub next: Option<Vec<String>>, // TODO min 1
    pub parents: Option<Vec<String>>, // TODO min 1
    pub children: Option<Vec<String>> // TODO min 1
}

#[derive(Debug, Deserialize)]
pub struct WorkZoneRoadEvent {
    // TODO
}

#[derive(Debug, Deserialize)]
pub struct DetourRoadEvent {
    // TODO
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    // TODO
}

#[derive(Debug, Deserialize)]
pub struct FeedInfo {
    pub update_date: String, // TODO validate
    pub version: String,
    pub publisher: String,
    pub data_sources: Vec<FeedDataSource>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub update_frequency: Option<i64>,
    pub license: Option<String> // TODO enum constrained
}

#[derive(Debug, Deserialize)]
pub struct FeedDataSource {
    pub data_source_id: String,
    pub organization_name: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub update_frequency: Option<i64>,
    pub update_date: Option<String>, // TODO validate
    pub lrs_type: Option<String>, // DEPRECATED
    pub lrs_url: Option<String>, // DEPRECATED
    pub location_verify_method: Option<String> // DEPRECATED
}