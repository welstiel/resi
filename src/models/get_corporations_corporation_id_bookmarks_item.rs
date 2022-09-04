/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdBookmarksItem : Optional object that is returned if a bookmark was made on a particular item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdBookmarksItem {
    /// item_id integer
    #[serde(rename = "item_id")]
    pub item_id: i64,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCorporationsCorporationIdBookmarksItem {
    /// Optional object that is returned if a bookmark was made on a particular item.
    pub fn new(item_id: i64, type_id: i32) -> GetCorporationsCorporationIdBookmarksItem {
        GetCorporationsCorporationIdBookmarksItem {
            item_id,
            type_id,
        }
    }
}


