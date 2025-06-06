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

/// DefaultUserProfile : Default serializer used for user profile. It will use these:  * User fields * :ref:`user-hidden-fields-setting` setting * :ref:`user-public-fields-setting` setting * :ref:`user-editable-fields-setting` setting  to automagically generate the required serializer fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultUserProfile {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "id")]
    pub id: String,
    /// First name (optional)
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (optional)
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl DefaultUserProfile {
    /// Default serializer used for user profile. It will use these:  * User fields * :ref:`user-hidden-fields-setting` setting * :ref:`user-public-fields-setting` setting * :ref:`user-editable-fields-setting` setting  to automagically generate the required serializer fields.
    pub fn new(email: String, id: String) -> DefaultUserProfile {
        DefaultUserProfile {
            email,
            id,
            first_name: None,
            last_name: None,
        }
    }
}

