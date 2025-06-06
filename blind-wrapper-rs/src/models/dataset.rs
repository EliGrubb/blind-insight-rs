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

/// Dataset : HyperlinkedModelSerializer with `id` as the first field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "organization")]
    pub organization: String,
    /// A human-readable name for this dataset.
    #[serde(rename = "name")]
    pub name: String,
    /// A unique slug for this dataset.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// A longer description of the dataset.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Dataset {
    /// HyperlinkedModelSerializer with `id` as the first field.
    pub fn new(id: String, url: String, organization: String, name: String) -> Dataset {
        Dataset {
            id,
            url,
            organization,
            name,
            slug: None,
            description: None,
        }
    }
}

