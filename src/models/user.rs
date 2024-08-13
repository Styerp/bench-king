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
    pub avatar: String
}