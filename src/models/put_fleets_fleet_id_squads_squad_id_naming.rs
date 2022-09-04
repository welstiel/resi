/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutFleetsFleetIdSquadsSquadIdNaming : naming object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFleetsFleetIdSquadsSquadIdNaming {
    /// name string
    #[serde(rename = "name")]
    pub name: String,
}

impl PutFleetsFleetIdSquadsSquadIdNaming {
    /// naming object
    pub fn new(name: String) -> PutFleetsFleetIdSquadsSquadIdNaming {
        PutFleetsFleetIdSquadsSquadIdNaming {
            name,
        }
    }
}

