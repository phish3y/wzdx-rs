use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl PartialEq for FeedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.update_date == other.update_date &&
        self.version == other.version &&
        self.publisher == other.publisher &&
        self.data_sources == other.data_sources &&
        self.contact_name == other.contact_name &&
        self.update_date == other.update_date &&
        self.contact_email == other.contact_email &&
        self.update_frequency == other.update_frequency &&
        self.license == other.license
    }
}

impl Eq for FeedInfo {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeedDataSource {
    pub data_source_id: String,
    pub organization_name: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub update_frequency: Option<u64>,
    pub update_date: Option<String>,
    #[deprecated]
    pub lrs_type: Option<String>,
    #[deprecated]
    pub lrs_url: Option<String>,
    #[deprecated]
    pub location_verify_method: Option<String>
}

impl PartialEq for FeedDataSource {
    #[allow(deprecated)]
    fn eq(&self, other: &Self) -> bool {
        self.data_source_id == other.data_source_id &&
        self.organization_name == other.organization_name &&
        self.contact_name == other.contact_name &&
        self.contact_email == other.contact_email &&
        self.update_frequency == other.update_frequency &&
        self.update_date == other.update_date &&
        self.lrs_type == other.lrs_type &&
        self.lrs_url == other.lrs_url &&
        self.location_verify_method == other.location_verify_method
    }
}

impl Eq for FeedDataSource {}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_fi_eq() {
        let fds1 = FeedDataSource {
            data_source_id: "id 1".to_string(),
            organization_name: "org name 1".to_string(),
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: None,
            update_date: None,
            lrs_type: None,
            lrs_url: None,
            location_verify_method: None
        };
        let fds2 = FeedDataSource {
            data_source_id: "id 2".to_string(),
            organization_name: "org name 1".to_string(),
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: None,
            update_date: None,
            lrs_type: None,
            lrs_url: None,
            location_verify_method: None
        };

        let fi1 = FeedInfo {
            update_date: "2024-07-25T17:37:00Z".to_string(),
            version: "1".to_string(),
            publisher: "pub 1".to_string(),
            data_sources: vec![fds1.clone()],
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: Some(5),
            license: None
        };
        let fi2 = FeedInfo {
            update_date: "2024-07-25T17:37:00Z".to_string(),
            version: "1".to_string(),
            publisher: "pub 1".to_string(),
            data_sources: vec![fds1.clone()],
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: Some(5),
            license: None
        };
        let fi3 = FeedInfo {
            update_date: "2024-07-25T17:37:00Z".to_string(),
            version: "1".to_string(),
            publisher: "pub 1".to_string(),
            data_sources: vec![fds1.clone()],
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: Some(5),
            license: None
        };
        let fi4 = FeedInfo {
            update_date: "2024-07-25T17:37:00Z".to_string(),
            version: "2".to_string(),
            publisher: "pub 1".to_string(),
            data_sources: vec![fds1.clone()],
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: Some(5),
            license: None
        };
        let fi5 = FeedInfo {
            update_date: "2024-07-25T17:37:00Z".to_string(),
            version: "2".to_string(),
            publisher: "pub 1".to_string(),
            data_sources: vec![fds1.clone(), fds1.clone()],
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: Some(5),
            license: None
        };
        let fi6 = FeedInfo {
            update_date: "2024-07-25T17:37:00Z".to_string(),
            version: "2".to_string(),
            publisher: "pub 1".to_string(),
            data_sources: vec![fds2.clone()],
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: Some(5),
            license: None
        };


        assert_eq!(fi1.eq(&fi2), true);
        assert_eq!(fi2.eq(&fi3), true);
        assert_eq!(fi1.eq(&fi3), true);
        assert_eq!(fi3.eq(&fi1), true);
        assert_eq!(fi1.eq(&fi4), false);
        assert_eq!(fi4.eq(&fi1), false);
        assert_eq!(fi1.eq(&fi5), false);
        assert_eq!(fi5.eq(&fi1), false);
        assert_eq!(fi1.eq(&fi6), false);
        assert_eq!(fi6.eq(&fi1), false);
    }

    #[test]
    #[allow(deprecated)]
    fn test_fds_eq() {
        let fds1 = FeedDataSource {
            data_source_id: "id 1".to_string(),
            organization_name: "org name 1".to_string(),
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: None,
            update_date: None,
            lrs_type: None,
            lrs_url: None,
            location_verify_method: None
        };

        let fds2 = FeedDataSource {
            data_source_id: "id 1".to_string(),
            organization_name: "org name 1".to_string(),
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: None,
            update_date: None,
            lrs_type: None,
            lrs_url: None,
            location_verify_method: None
        };

        let fds3 = FeedDataSource {
            data_source_id: "id 1".to_string(),
            organization_name: "org name 1".to_string(),
            contact_name: Some("contact name 1".to_string()),
            contact_email: None,
            update_frequency: None,
            update_date: None,
            lrs_type: None,
            lrs_url: None,
            location_verify_method: None
        };

        let fds4 = FeedDataSource {
            data_source_id: "id 2".to_string(),
            organization_name: "org name 2".to_string(),
            contact_name: Some("contact name 2".to_string()),
            contact_email: None,
            update_frequency: None,
            update_date: None,
            lrs_type: None,
            lrs_url: None,
            location_verify_method: None
        };

        assert_eq!(fds1.eq(&fds2), true);
        assert_eq!(fds2.eq(&fds3), true);
        assert_eq!(fds1.eq(&fds3), true);
        assert_eq!(fds3.eq(&fds1), true);
        assert_eq!(fds1.eq(&fds4), false);
        assert_eq!(fds4.eq(&fds1), false);
    }
}