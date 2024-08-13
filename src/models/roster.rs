use serde::{Deserialize, Serialize};

// {
//     "starters": ["2307", "2257", "4034", "147", "642", "4039", "515", "4149", "DET"],
//     "settings": {
//       "wins": 5,
//       "waiver_position": 7,
//       "waiver_budget_used": 0,
//       "total_moves": 0,
//       "ties": 0,
//       "losses": 9,
//       "fpts_decimal": 78,
//       "fpts_against_decimal": 32,
//       "fpts_against": 1670,
//       "fpts": 1617
//     },
//     "roster_id": 1,
//     "reserve": [],
//     "players": ["1046", "138", "147", "2257", "2307", "2319", "4034", "4039", "4040", "4149", "421", "515", "642", "745", "DET"],
//     "owner_id": "188815879448829952",
//     "league_id": "206827432160788480"
//   }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Roster {
    pub starters: Vec<String>,
    pub settings: RosterSettings,
    pub roster_id: u32,
    pub reserve: Vec<String>,
    pub players: Vec<String>,
    pub owner_id: String,
    pub league_id: String
}

pub struct RosterSettings {
    pub wins: u16,
    pub waiver_position: u16,
    pub waiver_budget_used: u128,
    pub total_moves: u128,
    pub ties: u16,
    pub losses: u16,
    pub fpts_decimal: f64,
    pub fpts_against_decimal: f64,
    pub fpts_against: u64,
    pub fpts: u64, 
}