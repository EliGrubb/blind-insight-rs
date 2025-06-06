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
pub struct IndexedRecordHashedLabelMapValue {
    #[serde(rename = "field_schema", skip_serializing_if = "Option::is_none")]
    pub field_schema: Option<Box<models::IndexedRecordHashedLabelMapValueFieldSchema>>,
    #[serde(rename = "hashed_value", skip_serializing_if = "Option::is_none")]
    pub hashed_value: Option<String>,
}

impl IndexedRecordHashedLabelMapValue {
    pub fn new() -> IndexedRecordHashedLabelMapValue {
        IndexedRecordHashedLabelMapValue {
            field_schema: None,
            hashed_value: None,
        }
    }
}

