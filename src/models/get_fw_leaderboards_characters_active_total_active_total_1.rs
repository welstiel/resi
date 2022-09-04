/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCharactersActiveTotalActiveTotal1 : active_total object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersActiveTotalActiveTotal1 {
    /// Amount of victory points
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// character_id integer
    #[serde(rename = "character_id", skip_serializing_if = "Option::is_none")]
    pub character_id: Option<i32>,
}

impl GetFwLeaderboardsCharactersActiveTotalActiveTotal1 {
    /// active_total object
    pub fn new() -> GetFwLeaderboardsCharactersActiveTotalActiveTotal1 {
        GetFwLeaderboardsCharactersActiveTotalActiveTotal1 {
            amount: None,
            character_id: None,
        }
    }
}

