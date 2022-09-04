/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetAlliancesAllianceIdIconsNotFound : No image server for this datasource



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAlliancesAllianceIdIconsNotFound {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetAlliancesAllianceIdIconsNotFound {
    /// No image server for this datasource
    pub fn new() -> GetAlliancesAllianceIdIconsNotFound {
        GetAlliancesAllianceIdIconsNotFound {
            error: None,
        }
    }
}


