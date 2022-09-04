/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetDogmaDynamicItemsTypeIdItemIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDogmaDynamicItemsTypeIdItemIdOk {
    /// The ID of the character who created the item
    #[serde(rename = "created_by")]
    pub created_by: i32,
    /// dogma_attributes array
    #[serde(rename = "dogma_attributes")]
    pub dogma_attributes: Vec<crate::models::GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute>,
    /// dogma_effects array
    #[serde(rename = "dogma_effects")]
    pub dogma_effects: Vec<crate::models::GetDogmaDynamicItemsTypeIdItemIdDogmaEffect>,
    /// The type ID of the mutator used to generate the dynamic item.
    #[serde(rename = "mutator_type_id")]
    pub mutator_type_id: i32,
    /// The type ID of the source item the mutator was applied to create the dynamic item.
    #[serde(rename = "source_type_id")]
    pub source_type_id: i32,
}

impl GetDogmaDynamicItemsTypeIdItemIdOk {
    /// 200 ok object
    pub fn new(created_by: i32, dogma_attributes: Vec<crate::models::GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute>, dogma_effects: Vec<crate::models::GetDogmaDynamicItemsTypeIdItemIdDogmaEffect>, mutator_type_id: i32, source_type_id: i32) -> GetDogmaDynamicItemsTypeIdItemIdOk {
        GetDogmaDynamicItemsTypeIdItemIdOk {
            created_by,
            dogma_attributes,
            dogma_effects,
            mutator_type_id,
            source_type_id,
        }
    }
}


