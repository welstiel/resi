/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdMedals200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMedals200Ok {
    /// corporation_id integer
    #[serde(rename = "corporation_id")]
    pub corporation_id: i32,
    /// date string
    #[serde(rename = "date")]
    pub date: String,
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// graphics array
    #[serde(rename = "graphics")]
    pub graphics: Vec<crate::models::GetCharactersCharacterIdMedalsGraphic>,
    /// issuer_id integer
    #[serde(rename = "issuer_id")]
    pub issuer_id: i32,
    /// medal_id integer
    #[serde(rename = "medal_id")]
    pub medal_id: i32,
    /// reason string
    #[serde(rename = "reason")]
    pub reason: String,
    /// status string
    #[serde(rename = "status")]
    pub status: Status,
    /// title string
    #[serde(rename = "title")]
    pub title: String,
}

impl GetCharactersCharacterIdMedals200Ok {
    /// 200 ok object
    pub fn new(corporation_id: i32, date: String, description: String, graphics: Vec<crate::models::GetCharactersCharacterIdMedalsGraphic>, issuer_id: i32, medal_id: i32, reason: String, status: Status, title: String) -> GetCharactersCharacterIdMedals200Ok {
        GetCharactersCharacterIdMedals200Ok {
            corporation_id,
            date,
            description,
            graphics,
            issuer_id,
            medal_id,
            reason,
            status,
            title,
        }
    }
}

/// status string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for Status {
    fn default() -> Status {
        Self::Public
    }
}

