/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCorporationsVictoryPoints : Top 10 rankings of corporations by victory points from yesterday, last week and in total



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsVictoryPoints {
    /// Top 10 ranking of corporations active in faction warfare by total victory points. A corporation is considered \"active\" if they have participated in faction warfare in the past 14 days
    #[serde(rename = "active_total")]
    pub active_total: Vec<crate::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1>,
    /// Top 10 ranking of corporations by victory points in the past week
    #[serde(rename = "last_week")]
    pub last_week: Vec<crate::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1>,
    /// Top 10 ranking of corporations by victory points in the past day
    #[serde(rename = "yesterday")]
    pub yesterday: Vec<crate::models::GetFwLeaderboardsCorporationsYesterdayYesterday1>,
}

impl GetFwLeaderboardsCorporationsVictoryPoints {
    /// Top 10 rankings of corporations by victory points from yesterday, last week and in total
    pub fn new(active_total: Vec<crate::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1>, last_week: Vec<crate::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1>, yesterday: Vec<crate::models::GetFwLeaderboardsCorporationsYesterdayYesterday1>) -> GetFwLeaderboardsCorporationsVictoryPoints {
        GetFwLeaderboardsCorporationsVictoryPoints {
            active_total,
            last_week,
            yesterday,
        }
    }
}


