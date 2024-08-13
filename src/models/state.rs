use serde::{Deserialize, Serialize};

// {
//     "week": 2,
//     "leg": 0,
//     "season": "2024",
//     "season_type": "pre",
//     "league_season": "2024",
//     "previous_season": "2023",
//     "season_start_date": "2024-08-01",
//     "display_week": 2,
//     "league_create_season": "2024",
//     "season_has_scores": true
//   }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct State {
    pub week: u8,
    pub leg: u8,
    pub season: String,
    pub season_type: String,
    pub league_season: String,
    pub previous_season: String,
    pub season_start_date: String,
    pub display_week: u8,
    pub league_create_season: String,
    pub season_has_scores: bool,
}
