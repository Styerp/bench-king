use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum RosterPosition {
    DB,
    DEF,
    DL,
    K,
    LB,
    LEO,
    LS,
    OG,
    OL,
    OT,
    P,
    QB,
    RB,
    TE,
    WR,
    FLEX,
    WRRB_FLEX,
    BN,
    KP,
}

impl RosterPosition {
    pub fn value(&self) -> Vec<String> {
        match self {
            RosterPosition::BN => vec!["BN".to_string()],   // Bench
            RosterPosition::DB => vec!["DB".to_string()],   // Defensive Back
            RosterPosition::DEF => vec!["DEF".to_string()], // Defense (Team)
            RosterPosition::DL => vec!["DL".to_string()],   // Defensive Line
            RosterPosition::FLEX => vec!["WR".to_string(), "RB".to_string(), "TE".to_string()], // WR/RB/TE Flex
            RosterPosition::K => vec!["K".to_string()], // Kicker
            RosterPosition::KP => vec!["K/P".to_string()], // Kicker/Punter
            RosterPosition::LB => vec!["LB".to_string()], // Linebacker
            RosterPosition::LEO => vec!["LEO".to_string()], // TODO: Is LEO included in fanatsy matchup data? LEO ~= LB + DE
            RosterPosition::LS => vec!["LS".to_string()],   // Long Snapper
            RosterPosition::OG => vec!["OG".to_string()],   // Offensive Guard
            RosterPosition::OL => vec!["OL".to_string()],   // Offensive Line
            RosterPosition::OT => vec!["OT".to_string()],   // Offensive Tackle
            RosterPosition::P => vec!["P".to_string()],     // Punter
            RosterPosition::QB => vec!["QB".to_string()],   // Quarterback
            RosterPosition::RB => vec!["RB".to_string()],   // Running Back
            RosterPosition::TE => vec!["TE".to_string()],   // Tight End
            RosterPosition::WR => vec!["WR".to_string()],   // Wide Receiver
            RosterPosition::WRRB_FLEX => vec!["WR".to_string(), "RB".to_string()], // WR/RB Flex
        }
    }
}
