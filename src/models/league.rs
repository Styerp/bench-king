use serde::{Deserialize, Serialize};

use super::Settings;
// {
//     "total_rosters": 12,
//     "status": "pre_draft", // can also be "drafting", "in_season", or "complete"
//     "sport": "nfl",
//     "settings": { settings object },
//     "season_type": "regular",
//     "season": "2018",
//     "scoring_settings": { scoring_settings object },
//     "roster_positions": [ roster positions array ],
//     "previous_league_id": "198946952535085056",
//     "name": "Sleeperbot Friends League",
//     "league_id": "289646328504385536",
//     "draft_id": "289646328508579840",
//     "avatar": "efaefa889ae24046a53265a3c71b8b64"
//   }

pub enum LeagueStatus {
    pre_draft,
    drafting,
    in_season,
    complete
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct League {
    pub total_rosters: u32,
    pub status: LeagueStatus,
    pub sport: String,
    pub settings: Settings,
    pub season_type: String,
    pub season: u32,
    pub scoring_settings: ScoringSettings,
    pub roster_positions: RosterPostions,
    pub previous_league_id: u128,
    pub name: String,
    pub league_id: String,
    pub draft_id: String,
    pub avatar: String

}