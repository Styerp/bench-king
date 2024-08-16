use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum RosterPosition {
    BN,
    C,
    CB,
    DB,
    DE,
    DEF,
    DL,
    DT,
    FB,
    FLEX,
    FS,
    G,
    ILB,
    K,
    #[serde(alias = "K/P")]
    KP,
    LB,
    LEO,
    LS,
    NT,
    OG,
    OL,
    OLB,
    OT,
    P,
    QB,
    RB,
    S,
    SS,
    T,
    TE,
    WR,
    WRRB_FLEX,
}

impl RosterPosition {
    pub fn value(&self) -> Vec<String> {
        match self {
            RosterPosition::BN => vec!["BN".to_string()],   // Bench
            RosterPosition::C => vec!["C".to_string()],     // Center
            RosterPosition::CB => vec!["CB".to_string()],   // Cornerback
            RosterPosition::DB => vec!["DB".to_string()],   // Defensive Back
            RosterPosition::DE => vec!["DE".to_string()],   // Defensive End
            RosterPosition::DEF => vec!["DEF".to_string()], // Defense (Team)
            RosterPosition::DL => vec!["DL".to_string()],   // Defensive Line
            RosterPosition::DT => vec!["DT".to_string()],   // Defensive Tackle
            RosterPosition::FB => vec!["FB".to_string()],   // Fullback
            RosterPosition::FLEX => vec!["WR".to_string(), "RB".to_string(), "TE".to_string()], // WR/RB/TE Flex
            RosterPosition::FS => vec!["FS".to_string()],   // Free Safety
            RosterPosition::G => vec!["G".to_string()],     // Guard
            RosterPosition::ILB => vec!["ILB".to_string()], // Inside Linebacker
            RosterPosition::K => vec!["K".to_string()], // Kicker
            RosterPosition::KP => vec!["K/P".to_string()], // Kicker/Punter
            RosterPosition::LB => vec!["LB".to_string()], // Linebacker
            RosterPosition::LEO => vec!["LEO".to_string()], // TODO: Is LEO included in fanatsy matchup data? LEO ~= LB + DE
            RosterPosition::LS => vec!["LS".to_string()],   // Long Snapper
            RosterPosition::NT => vec!["NT".to_string()],   // Nose Tackle
            RosterPosition::OG => vec!["OG".to_string()],   // Offensive Guard
            RosterPosition::OL => vec!["OL".to_string()],   // Offensive Line
            RosterPosition::OLB => vec!["OLB".to_string()], // Outside Linebacker
            RosterPosition::OT => vec!["OT".to_string()],   // Offensive Tackle
            RosterPosition::P => vec!["P".to_string()],     // Punter
            RosterPosition::QB => vec!["QB".to_string()],   // Quarterback
            RosterPosition::RB => vec!["RB".to_string()],   // Running Back
            RosterPosition::S => vec!["S".to_string()],     // Safety
            RosterPosition::SS => vec!["SS".to_string()],   // Strong Safety
            RosterPosition::T => vec!["T".to_string()],     // Tackle
            RosterPosition::TE => vec!["TE".to_string()],   // Tight End
            RosterPosition::WR => vec!["WR".to_string()],   // Wide Receiver
            RosterPosition::WRRB_FLEX => vec!["WR".to_string(), "RB".to_string()], // WR/RB Flex
        }
    }
}
