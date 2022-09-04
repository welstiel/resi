/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdContacts200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContacts200Ok {
    /// contact_id integer
    #[serde(rename = "contact_id")]
    pub contact_id: i32,
    /// contact_type string
    #[serde(rename = "contact_type")]
    pub contact_type: ContactType,
    /// Whether this contact is being watched
    #[serde(rename = "is_watched", skip_serializing_if = "Option::is_none")]
    pub is_watched: Option<bool>,
    /// label_ids array
    #[serde(rename = "label_ids", skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<i64>>,
    /// Standing of the contact
    #[serde(rename = "standing")]
    pub standing: f32,
}

impl GetCorporationsCorporationIdContacts200Ok {
    /// 200 ok object
    pub fn new(contact_id: i32, contact_type: ContactType, standing: f32) -> GetCorporationsCorporationIdContacts200Ok {
        GetCorporationsCorporationIdContacts200Ok {
            contact_id,
            contact_type,
            is_watched: None,
            label_ids: None,
            standing,
        }
    }
}

/// contact_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContactType {
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "alliance")]
    Alliance,
    #[serde(rename = "faction")]
    Faction,
}

impl Default for ContactType {
    fn default() -> ContactType {
        Self::Character
    }
}

