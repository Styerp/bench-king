use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// {
//     "3086": {
//       "hashtag": "#TomBrady-NFL-NE-12",
//       "depth_chart_position": 1,
//       "status": "Active",
//       "sport": "nfl",
//       "fantasy_positions": ["QB"],
//       "number": 12,
//       "search_last_name": "brady",
//       "injury_start_date": null,
//       "weight": "220",
//       "position": "QB",
//       "practice_participation": null,
//       "sportradar_id": "",
//       "team": "NE",
//       "last_name": "Brady",
//       "college": "Michigan",
//       "fantasy_data_id":17836,
//       "injury_status":null,
//       "player_id":"3086",
//       "height": "6'4\"",
//       "search_full_name": "tombrady",
//       "age": 40,
//       "stats_id": "",
//       "birth_country": "United States",
//       "espn_id": "",
//       "search_rank": 24,
//       "first_name": "Tom",
//       "depth_chart_order": 1,
//       "years_exp": 14,
//       "rotowire_id": null,
//       "rotoworld_id": 8356,
//       "search_first_name": "tom",
//       "yahoo_id": null
//     },
//     ...
//   }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerDetails {
    pub active: bool,
    pub age: u8,
    pub birth_city: String,
    pub birth_country: String,
    pub birth_date: String,
    pub birth_state: String,
    pub college: String,
    pub competitions: Vec<String>,
    pub depth_chart_order: u16,
    pub depth_chart_position: u16,
    pub espn_id: String,
    pub fantasy_data_id: String,
    pub fantasy_positions: Vec<String>, // TODO: Enum of roster positions
    pub first_name: String,
    pub full_name: String,
    pub gsis_id: String,
    pub hashtag: String,
    pub height: String,
    pub high_school: String,
    pub injury_body_part: String,
    pub injury_notes: String,
    pub injury_start_date: String,
    pub injury_status: String,
    pub last_name: String,
    pub metadata: PlayerMetadata,
    pub news_updated: u128,
    pub number: u8,
    pub oddsjam_id: String,
    pub opta_id: String,
    pub pandascore_id: String,
    pub player_id: String,
    pub position: String, // Roster position
    pub practice_participation: String,
    pub rotowire_id: u16,
    pub rotoworld_id: u16,
    pub search_first_name: String,
    pub search_full_name: String,
    pub search_last_name: String,
    pub search_rank: u16,
    pub sport: String,
    pub sportradar_id: String,
    pub stats_id: String,
    pub status: String,
    pub swish_id: u16,
    pub team: String,
    pub team_abbr: String,
    pub weight: String,
    pub yahoo_id: String,
    pub years_exp: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerMetadata {
    pub channel_id: String,
    pub injury_override_off_2021_0: Option<String>,
    pub injury_override_regular_2020_1: Option<String>,
    pub injury_override_regular_2020_10: Option<String>,
    pub injury_override_regular_2020_5: Option<String>,
    pub injury_override_regular_2020_8: Option<String>,
    pub injury_override_regular_2021_1: Option<String>,
    pub injury_override_regular_2021_3: Option<String>,
    pub injury_override_regular_2021_4: Option<String>,
    pub injury_override_regular_2022_1: Option<String>,
    pub name: Option<String>,
    pub override_active: Option<String>,
    pub rookie_year: Option<String>,
    pub source_id: Option<String>,
    pub years_exp_shift: u8,
}

pub type Player = HashMap<String, PlayerDetails>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrendingPlayer {
    pub player_id: String,
    pub count: u32,
}
