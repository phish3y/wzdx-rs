use serde::{Deserialize, Serialize};

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