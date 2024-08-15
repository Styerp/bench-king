use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type UserId = String;
//{
// "username": "sleeperuser",
// "user_id": "12345678",
// "display_name": "SleeperUser",
// "avatar": "cc12ec49965eb7856f84d71cf85306af"
//  }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub user_id: UserId,
    pub display_name: String,
    pub avatar: String,
    pub cookies: Option<String>,
    pub created: Option<String>,
    pub currencies: Option<String>,
    pub data_updated: Option<String>,
    pub deleted: Option<String>,
    pub email: Option<String>,
    pub metadata: Option<String>,
    pub notifications: Option<String>,
    pub pending: Option<String>,
    pub phone: Option<String>,
    pub real_name: Option<String>,
    pub solicitable: Option<String>,
    pub summoner_name: Option<String>,
    pub summoner_realm: Option<String>,
    pub token: Option<String>,
    pub verification: Option<String>,
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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LeagueUser {
    pub username: String,
    pub user_id: UserId,
    pub display_name: String,
    pub avatar: String,
    pub metadata: LeagueUserMetadata,
    pub is_owner: Option<bool>,
    pub is_bot: bool,
    pub settings: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LeagueUserMetadata {
    pub team_name: Option<String>,
    // pn == Push Notification
    pub allow_pn: Option<String>,
    pub mention_pn: Option<String>,
    // Link if custom, otherwise id
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

impl Display for LeagueUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "League User: {}", self)
    }
}
impl Display for LeagueUserMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "League User Metadata: {}", self)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {}", self)
    }
}
