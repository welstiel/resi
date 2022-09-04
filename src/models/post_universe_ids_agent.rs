/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsAgent : agent object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUniverseIdsAgent {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsAgent {
    /// agent object
    pub fn new() -> PostUniverseIdsAgent {
        PostUniverseIdsAgent {
            id: None,
            name: None,
        }
    }
}


