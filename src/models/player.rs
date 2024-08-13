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
    pub hashtag: String,
    pub depth_chart_position: u8,
    pub status: String,
    pub sport: String,
    pub fantasy_positions: Vec<String>, // TODO: Enum of roster positions
    pub number: u8,
    pub search_last_name: String,
    pub injury_start_date: String,
    pub weight: u16,
    pub position: String,
    pub practice_participation: String,
    pub sportradar_id: String,
    pub team: String,
    pub last_name: String,
    pub college: String,
    pub fantasy_data_id: String,
    pub injury_status: String,
    pub player_id: String,
    pub height: String,
    pub search_full_name: String,
    pub age: u8,
    pub stats_id: String,
    pub birth_country: String,
    pub espn_id: String,
    pub search_rank: u16,
    pub depth_chart_order: u16,
    pub years_exp: u8,
    pub rotowire_id: u16,
    pub rotoworld_id: u16,
    pub search_first_name: String,
    pub yahoo_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player(HashMap<String, PlayerDetails>);
