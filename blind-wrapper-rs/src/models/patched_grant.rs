/*
 * Blind Insight REST API
 *
 * End-to-end encrypted datastore
 *
 * The version of the OpenAPI document: 10.6.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedGrant : HyperlinkedModelSerializer with `id` as the first field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGrant {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<String>>,
    /// A human-readable name for this grant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The codename for this grant.
    #[serde(rename = "codename", skip_serializing_if = "Option::is_none")]
    pub codename: Option<String>,
    /// Field names of keys to request. None means allows all fields & query key; or a mapping of specific field names: `{'field_name': True, ...}`
    #[serde(rename = "field_names", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub field_names: Option<Option<serde_json::Value>>,
    /// Whether this permission allows the user to share the key(s) assigned to this permission.
    #[serde(rename = "can_share_keys", skip_serializing_if = "Option::is_none")]
    pub can_share_keys: Option<bool>,
    /// Whether this permission allows the user to create records.
    #[serde(rename = "can_create_records", skip_serializing_if = "Option::is_none")]
    pub can_create_records: Option<bool>,
    /// The dataset that this grant belongs to.
    #[serde(rename = "dataset", skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    /// The schema that this grant belongs to.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

impl PatchedGrant {
    /// HyperlinkedModelSerializer with `id` as the first field.
    pub fn new() -> PatchedGrant {
        PatchedGrant {
            id: None,
            url: None,
            organization: None,
            teams: None,
            name: None,
            codename: None,
            field_names: None,
            can_share_keys: None,
            can_create_records: None,
            dataset: None,
            schema: None,
        }
    }
}

