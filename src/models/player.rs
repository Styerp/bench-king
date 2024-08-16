use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::positions::RosterPosition;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerDetails {
    pub active: bool,
    pub age: Option<u8>,
    pub birth_city: Option<String>,
    pub birth_country: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_state: Option<String>,
    pub college: Option<String>,
    pub competitions: Option<Vec<String>>,
    pub depth_chart_order: Option<u16>,
    pub depth_chart_position: Option<String>,
    pub espn_id: Option<u32>,
    pub fantasy_data_id: Option<u16>,
    pub fantasy_positions: Option<Vec<RosterPosition>>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub gsis_id: Option<String>,
    pub hashtag: Option<String>,
    pub height: Option<String>,
    pub high_school: Option<String>,
    pub injury_body_part: Option<String>,
    pub injury_notes: Option<String>,
    pub injury_start_date: Option<NaiveDate>,
    pub injury_status: Option<String>,
    pub last_name: Option<String>,
    pub metadata: Option<PlayerMetadata>,
    pub news_updated: Option<u128>,
    pub number: Option<u8>,
    pub oddsjam_id: Option<String>,
    pub opta_id: Option<String>,
    pub pandascore_id: Option<String>,
    pub player_id: String,
    pub position: Option<RosterPosition>,
    pub practice_participation: Option<String>,
    pub rotowire_id: Option<u16>,
    pub rotoworld_id: Option<u16>,
    pub search_first_name: Option<String>,
    pub search_full_name: Option<String>,
    pub search_last_name: Option<String>,
    pub search_rank: Option<u32>,
    pub sport: String,
    pub sportradar_id: Option<String>,
    pub stats_id: Option<u32>,
    pub status: Option<String>,
    pub swish_id: Option<u32>,
    pub team: Option<String>,
    pub team_abbr: Option<String>,
    pub weight: Option<String>,
    pub yahoo_id: Option<u32>,
    pub years_exp: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerMetadata {
    pub channel_id: Option<String>,
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
    pub years_exp_shift: Option<String>,
}

pub type Players = HashMap<String, PlayerDetails>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrendingPlayer {
    pub player_id: String,
    pub count: u32,
}
