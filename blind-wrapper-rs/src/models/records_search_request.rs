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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordsSearchRequest {
    /// Schema ID to search within
    #[serde(rename = "schema")]
    pub schema: String,
    /// Array of label/value pairs to filter records
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<models::RecordsSearchRequestFiltersInner>>,
    /// Maximum number of records to return
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Number of records to skip
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl RecordsSearchRequest {
    pub fn new(schema: String) -> RecordsSearchRequest {
        RecordsSearchRequest {
            schema,
            filters: None,
            limit: None,
            offset: None,
        }
    }
}

