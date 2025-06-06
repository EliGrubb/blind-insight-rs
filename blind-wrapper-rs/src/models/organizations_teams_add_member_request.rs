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
pub struct OrganizationsTeamsAddMemberRequest {
    #[serde(rename = "id")]
    pub id: String,
}

impl OrganizationsTeamsAddMemberRequest {
    pub fn new(id: String) -> OrganizationsTeamsAddMemberRequest {
        OrganizationsTeamsAddMemberRequest {
            id,
        }
    }
}

