use serde::{Deserialize, Serialize};

//{
// "username": "sleeperuser",
// "user_id": "12345678",
// "display_name": "SleeperUser",
// "avatar": "cc12ec49965eb7856f84d71cf85306af"
//  }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub user_id: u128,
    pub display_name: String,
    pub avatar: String,
}

// {
//     "user_id": "<user_id>",
//     "username": "<username>",
//     "display_name": "<display_name>",
//     "avatar": "1233456789",
//     "metadata": {
//       "team_name": "Dezpacito"
//     },
//     "is_owner": true   // is commissioner (there can be multiple commissioners)
//   },
pub struct LeagueUser {
    pub username: String,
    pub user_id: u128,
    pub display_name: String,
    pub avatar: String,
    pub metadata: LeagueUserMetadata,
    pub is_owner: bool,
    pub is_bot: bool,
}

pub struct LeagueUserMetadata {
    pub team_name: Option<String>,
    // pn == Push Notification
    pub allow_pn: Option<String>,
    pub mention_pn: Option<String>,
    // Link to the image
    pub avatar: Option<String>,
    pub league_report_pn: Option<String>,
    pub mascot_message: Option<String>,
    pub player_like_pn: Option<String>,
    pub player_nickname_update: Option<String>,
    pub team_name_update: Option<String>,
    pub trade_block_pn: Option<String>,
    pub transaction_commissioner: Option<String>,
    pub transaction_free_agent: Option<String>,
    pub transaction_trade: Option<String>,
    pub transaction_waiver: Option<String>,
    pub user_message_pn: Option<String>,
}
