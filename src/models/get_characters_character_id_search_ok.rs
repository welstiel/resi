/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdSearchOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdSearchOk {
    /// agent array
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<Vec<i32>>,
    /// alliance array
    #[serde(rename = "alliance", skip_serializing_if = "Option::is_none")]
    pub alliance: Option<Vec<i32>>,
    /// character array
    #[serde(rename = "character", skip_serializing_if = "Option::is_none")]
    pub character: Option<Vec<i32>>,
    /// constellation array
    #[serde(rename = "constellation", skip_serializing_if = "Option::is_none")]
    pub constellation: Option<Vec<i32>>,
    /// corporation array
    #[serde(rename = "corporation", skip_serializing_if = "Option::is_none")]
    pub corporation: Option<Vec<i32>>,
    /// faction array
    #[serde(rename = "faction", skip_serializing_if = "Option::is_none")]
    pub faction: Option<Vec<i32>>,
    /// inventory_type array
    #[serde(rename = "inventory_type", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<Vec<i32>>,
    /// region array
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<i32>>,
    /// solar_system array
    #[serde(rename = "solar_system", skip_serializing_if = "Option::is_none")]
    pub solar_system: Option<Vec<i32>>,
    /// station array
    #[serde(rename = "station", skip_serializing_if = "Option::is_none")]
    pub station: Option<Vec<i32>>,
    /// structure array
    #[serde(rename = "structure", skip_serializing_if = "Option::is_none")]
    pub structure: Option<Vec<i64>>,
}

impl GetCharactersCharacterIdSearchOk {
    /// 200 ok object
    pub fn new() -> GetCharactersCharacterIdSearchOk {
        GetCharactersCharacterIdSearchOk {
            agent: None,
            alliance: None,
            character: None,
            constellation: None,
            corporation: None,
            faction: None,
            inventory_type: None,
            region: None,
            solar_system: None,
            station: None,
            structure: None,
        }
    }
}


