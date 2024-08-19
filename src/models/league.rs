use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{positions::RosterPosition, settings::ScoringSettings};

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub enum LeagueStatus {
//     PreDraft = "pre_draft",
//     Drafting = "drafting",
//     InSeason = "in_season",
//     Complete = "complete"
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct League {
    pub total_rosters: u32,
    pub status: String,
    pub sport: String,
    pub settings: LeagueSettings,
    pub season_type: String,
    pub season: String,
    pub scoring_settings: ScoringSettings,
    pub roster_positions: Vec<RosterPosition>,
    pub previous_league_id: Option<String>,
    pub name: String,
    pub league_id: String,
    pub draft_id: String,
    pub avatar: Option<String>,
    pub company_id: Option<String>,
}
impl Display for League {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "League: {}", self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LeagueSettings {
    pub bench_lock: u8,
    pub best_ball: Option<u32>,
    pub capacity_override: u16,
    pub commissioner_direct_invite: u16,
    pub daily_waivers: u16,
    pub daily_waivers_days: u32,
    pub daily_waivers_hour: u16,
    pub disable_adds: u32,
    pub disable_trades: Option<u16>,
    pub divisions: Option<u8>,
    pub draft_rounds: u8,
    pub league_average_match: u8,
    #[serde(rename = "type")]
    pub league_type: u16,
    pub leg: u8,
    pub max_keepers: u8,
    pub num_teams: u16,
    pub offseason_adds: u8,
    pub pick_trading: u16,
    pub playoff_round_type: u16,
    pub playoff_seed_type: u32,
    pub playoff_teams: u16,
    pub playoff_type: u16,
    pub playoff_week_start: u16,
    pub reserve_allow_cov: u8,
    pub reserve_allow_dnr: u16,
    pub reserve_allow_doubtful: u16,
    pub reserve_allow_na: u8,
    pub reserve_allow_out: u16,
    pub reserve_allow_sus: u16,
    pub reserve_slots: u8,
    pub start_week: u16,
    pub taxi_allow_vets: u16,
    pub taxi_deadline: u32,
    pub taxi_slots: u16,
    pub taxi_years: u16,
    pub trade_deadline: u16,
    pub trade_review_days: u8,
    pub veto_auto_poll: Option<u16>,
    pub veto_show_votes: Option<u16>,
    pub veto_votes_needed: Option<u16>,
    pub waiver_bid_min: Option<u32>,
    pub waiver_budget: u32,
    pub waiver_clear_days: u16,
    pub waiver_day_of_week: u8,
    pub waiver_type: u8,
}

impl Display for LeagueSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "League Settings: {}", self)
    }
}
