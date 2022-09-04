/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorLimited : Error limited model



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorLimited {
    /// Error limited message
    #[serde(rename = "error")]
    pub error: String,
}

impl ErrorLimited {
    /// Error limited model
    pub fn new(error: String) -> ErrorLimited {
        ErrorLimited {
            error,
        }
    }
}

