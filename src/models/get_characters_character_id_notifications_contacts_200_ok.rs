/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdNotificationsContacts200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdNotificationsContacts200Ok {
    /// message string
    #[serde(rename = "message")]
    pub message: String,
    /// notification_id integer
    #[serde(rename = "notification_id")]
    pub notification_id: i32,
    /// send_date string
    #[serde(rename = "send_date")]
    pub send_date: String,
    /// sender_character_id integer
    #[serde(rename = "sender_character_id")]
    pub sender_character_id: i32,
    /// A number representing the standing level the receiver has been added at by the sender. The standing levels are as follows: -10 -> Terrible | -5 -> Bad |  0 -> Neutral |  5 -> Good |  10 -> Excellent
    #[serde(rename = "standing_level")]
    pub standing_level: f32,
}

impl GetCharactersCharacterIdNotificationsContacts200Ok {
    /// 200 ok object
    pub fn new(message: String, notification_id: i32, send_date: String, sender_character_id: i32, standing_level: f32) -> GetCharactersCharacterIdNotificationsContacts200Ok {
        GetCharactersCharacterIdNotificationsContacts200Ok {
            message,
            notification_id,
            send_date,
            sender_character_id,
            standing_level,
        }
    }
}

