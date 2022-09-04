/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseTypesTypeIdNotFound : Not found



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseTypesTypeIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetUniverseTypesTypeIdNotFound {
    /// Not found
    pub fn new() -> GetUniverseTypesTypeIdNotFound {
        GetUniverseTypesTypeIdNotFound {
            error: None,
        }
    }
}


