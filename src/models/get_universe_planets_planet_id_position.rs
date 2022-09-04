/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniversePlanetsPlanetIdPosition : position object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniversePlanetsPlanetIdPosition {
    /// x number
    #[serde(rename = "x")]
    pub x: f64,
    /// y number
    #[serde(rename = "y")]
    pub y: f64,
    /// z number
    #[serde(rename = "z")]
    pub z: f64,
}

impl GetUniversePlanetsPlanetIdPosition {
    /// position object
    pub fn new(x: f64, y: f64, z: f64) -> GetUniversePlanetsPlanetIdPosition {
        GetUniversePlanetsPlanetIdPosition {
            x,
            y,
            z,
        }
    }
}


