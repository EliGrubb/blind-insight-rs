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

/// PatchedDataset : HyperlinkedModelSerializer with `id` as the first field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedDataset {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// A human-readable name for this dataset.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A unique slug for this dataset.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// A longer description of the dataset.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PatchedDataset {
    /// HyperlinkedModelSerializer with `id` as the first field.
    pub fn new() -> PatchedDataset {
        PatchedDataset {
            id: None,
            url: None,
            organization: None,
            name: None,
            slug: None,
            description: None,
        }
    }
}

