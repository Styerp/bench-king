use std::collections::HashMap;

use bench_king_sleeper::{
    client::SleeperClient,
    models::{
        matchup::Matchup,
        roster::Roster,
        user::{LeagueUser, UserId},
    },
};
use serde::{Deserialize, Serialize};

const LEAGUE_ID: &str = "1124926301107884032";
const THROUGH_WEEK: i32 = 14;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
struct SeasonPerformance {
    team_name: String,
    head_to_head_wins: i8,
    head_to_head_losses: i8,
    league_wins: i8,
    league_losses: i8,
    points_for: f32,
    points_against: f32,
}

impl Eq for SeasonPerformance {}

impl std::ops::Add for SeasonPerformance {
    type Output = SeasonPerformance;
    fn add(self, rhs: Self) -> Self::Output {
        SeasonPerformance {
            team_name: self.team_name,
            head_to_head_wins: self.head_to_head_wins + rhs.head_to_head_wins,
            head_to_head_losses: self.head_to_head_losses + rhs.head_to_head_losses,
            league_wins: self.league_wins + rhs.league_wins,
            league_losses: self.league_losses + rhs.league_losses,
            points_for: self.points_for + rhs.points_for,
            points_against: self.points_against + self.points_against,
        }
    }
}

impl std::ops::AddAssign for SeasonPerformance {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            team_name: self.team_name.clone(),
            head_to_head_wins: self.head_to_head_wins + rhs.head_to_head_wins,
            head_to_head_losses: self.head_to_head_losses + rhs.head_to_head_losses,
            league_wins: self.league_wins + rhs.league_wins,
            league_losses: self.league_losses + rhs.league_losses,
            points_for: self.points_for + rhs.points_for,
            points_against: self.points_against + rhs.points_against,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Standing {
    team_name: String,
    rank_league: usize,
    rank_head_to_head: usize,
    rank_combined: usize,
    season_performance: SeasonPerformance,
}

impl std::fmt::Display for Standing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Team {} ranked {} by head to head wins, {} by league wins, and {} combined.
    Rank delta: {}
    Records:
        {}-{} Head to Head
        {}-{} League
        {}-{} Combined
        Pts for :    {}
        Pts against: {}
            "#,
            self.team_name,
            self.rank_head_to_head + 1,
            self.rank_league + 1,
            self.rank_combined + 1,
            (self.rank_combined as i8 - self.rank_head_to_head as i8).abs(),
            self.season_performance.head_to_head_wins,
            self.season_performance.head_to_head_losses,
            self.season_performance.league_wins,
            self.season_performance.league_losses,
            self.season_performance.head_to_head_wins + self.season_performance.league_wins,
            self.season_performance.head_to_head_losses + self.season_performance.league_losses,
            self.season_performance.points_for,
            self.season_performance.points_against
        )
    }
}

fn calc_ranks(inputs: Vec<SeasonPerformance>) -> Vec<Standing> {
    let mut standings: HashMap<_, Standing> = HashMap::new();

    let mut head = inputs.clone();
    head.sort_by_key(|a| (-a.head_to_head_wins, -a.points_for as i32));

    let mut league = inputs.clone();
    league.sort_by_key(|a| (-a.league_wins, -a.points_for as i32));

    let mut comb = inputs.clone();
    comb.sort_by_key(|a| (-(a.league_wins + a.head_to_head_wins), -a.points_for as i32));
    for inp in &inputs {
        let hhr = head
            .iter()
            .position(|f| f.team_name == inp.team_name)
            .unwrap();
        let lhr = league
            .iter()
            .position(|f| f.team_name == inp.team_name)
            .unwrap();
        let cr = comb
            .iter()
            .position(|f| f.team_name == inp.team_name)
            .unwrap();

        standings.insert(&inp.team_name, {
            Standing {
                team_name: inp.team_name.clone(),
                rank_head_to_head: hhr,
                rank_combined: cr,
                rank_league: lhr,
                season_performance: inp.clone(),
            }
        });
    }
    let mut final_data: Vec<Standing> = vec![];
    for (_user, details) in standings {
        final_data.push(details)
    }
    final_data
}

fn calculate_week_performance(
    matchup: &Matchup,
    matchups: &Vec<Matchup>,
    rosters: &Vec<Roster>,
    teams: &Vec<LeagueUser>,
    median: f32,
) -> SeasonPerformance {
    let opp = matchups
        .iter()
        .find(|f| &f.matchup_id == &matchup.matchup_id && f.roster_id != matchup.roster_id)
        .unwrap()
        .clone();
    let roster = rosters
        .iter()
        .find(|r| &r.roster_id == &matchup.roster_id)
        .unwrap();
    let owner = teams.iter().find(|t| t.user_id == roster.owner_id).unwrap();
    let pts: f32 = matchup.starters_points.clone().iter().sum();
    let opp_pts = opp.starters_points.clone().iter().sum();
    let wk_perf = SeasonPerformance {
        team_name: owner.display_name.clone(),
        head_to_head_wins: if pts > opp_pts { 1 } else { 0 },
        head_to_head_losses: if pts < opp_pts { 1 } else { 0 },
        league_wins: if pts > median { 1 } else { 0 },
        league_losses: if pts < median { 1 } else { 0 },
        points_for: matchup.starters_points.iter().sum(),
        points_against: opp.starters_points.iter().sum(),
    };
    wk_perf
}

async fn calculate_season_performances(league_id: String) -> Vec<SeasonPerformance> {
    let client = SleeperClient::build();
    let teams = client.get_users_in_league(&league_id).await.unwrap();
    let rosters = client
        .get_rosters_in_league(&league_id)
        .await
        .unwrap();

    let mut data: HashMap<UserId, SeasonPerformance> = HashMap::new();
    for week in 1..(THROUGH_WEEK + 1) {
        let wk = client
            .get_league_matchups_for_week(&league_id, week)
            .await
            .unwrap();
        let mut median_setup: Vec<f32> = wk
            .iter()
            .map(|m| m.starters_points.clone().iter().sum())
            .collect();
        median_setup.sort_by_key(|a| (a * 1000.0) as i32);
        let median = (median_setup.get(6).unwrap() + median_setup.get(7).unwrap()) / 2.0;
        for matchup in &wk {
            let wk_perf = calculate_week_performance(matchup, &wk, &rosters, &teams, median);
            data.entry(wk_perf.team_name.clone())
                .and_modify(|r| *r += wk_perf.clone())
                .or_insert(wk_perf);
        }
    }
    let mut final_data: Vec<SeasonPerformance> = vec![];
    for (_user, details) in data {
        final_data.push(details)
    }
    final_data
}

#[tokio::main]
async fn main() {
    let perf = calculate_season_performances(LEAGUE_ID.to_owned()).await;
    let mut ranks = calc_ranks(perf);
    ranks.sort_by_key(|a| a.rank_head_to_head);
    for r in ranks {
        println!("{}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bench_king_sleeper::models::roster::RosterSettings;
    #[test]
    fn test_calculate_week_performance() {
        let matchups = vec![
            Matchup {
                matchup_id: 1,
                starters: Vec::new(),
                roster_id: 1,
                players: Vec::new(),
                points: 132.1,
                custom_points: Some(0.0),
                players_points: HashMap::from_iter(vec![("a".to_string(), 1.0)]),
                starters_points: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            },
            Matchup {
                matchup_id: 1,
                starters: Vec::new(),
                roster_id: 2,
                players: Vec::new(),
                points: 133.1,
                custom_points: Some(0.0),
                players_points: HashMap::from_iter(vec![("a".to_string(), 1.0)]),
                starters_points: vec![1.0, 2.0, 3.0, 4.0, 6.0],
            },
            Matchup {
                matchup_id: 2,
                starters: Vec::new(),
                roster_id: 3,
                players: Vec::new(),
                points: 132.1,
                custom_points: Some(0.0),
                players_points: HashMap::from_iter(vec![("a".to_string(), 1.0)]),
                starters_points: vec![1.0, 2.0, 3.0, 4.0, 17.0],
            },
            Matchup {
                matchup_id: 2,
                starters: Vec::new(),
                roster_id: 4,
                players: Vec::new(),
                points: 132.1,
                custom_points: Some(0.0),
                players_points: HashMap::from_iter(vec![("a".to_string(), 1.0)]),
                starters_points: vec![1.0, 2.0, 3.0],
            },
        ];
        let roster_settings = RosterSettings {
            wins: 0,
            waiver_position: 0,
            waiver_budget_used: 0100,
            total_moves: 0100,
            ties: 0,
            losses: 0,
            fpts_decimal: Some(0.0),
            fpts_against_decimal: Some(0.0),
            fpts_against: Some(0),
            fpts: 0,
            division: Some(1),
        };
        let rosters = vec![
            Roster {
                roster_id: 1,
                owner_id: "ME!".to_string(),
                starters: Vec::new(),
                settings: roster_settings.clone(),
                co_owners: None,
                reserve: None,
                players: None,
                player_map: None,
                league_id: LEAGUE_ID.to_string(),
                keepers: None,
                metadata: None,
            },
            Roster {
                roster_id: 2,
                owner_id: "YOU!".to_string(),
                starters: Vec::new(),
                settings: roster_settings.clone(),
                co_owners: None,
                reserve: None,
                players: None,
                player_map: None,
                league_id: LEAGUE_ID.to_string(),
                keepers: None,
                metadata: None,
            },
            Roster {
                roster_id: 3,
                owner_id: "EVERYONE!".to_string(),
                starters: Vec::new(),
                settings: roster_settings.clone(),
                co_owners: None,
                reserve: None,
                players: None,
                player_map: None,
                league_id: LEAGUE_ID.to_string(),
                keepers: None,
                metadata: None,
            },
            Roster {
                roster_id: 4,
                owner_id: "McJesus!".to_string(),
                starters: Vec::new(),
                settings: roster_settings,
                co_owners: None,
                reserve: None,
                players: None,
                player_map: None,
                league_id: LEAGUE_ID.to_string(),
                keepers: None,
                metadata: None,
            },
        ];
        let teams = vec![
            LeagueUser {
                username: Some("Someone".to_string()),
                user_id: "ME!".to_string(),
                display_name: "Pete'sFarts".to_string(),
                avatar: "123".to_string(),
                metadata: None,
                is_owner: Some(true),
                is_bot: false,
                settings: None,
            },
            LeagueUser {
                username: Some("Someone".to_string()),
                user_id: "YOU!".to_string(),
                display_name: "YOU!".to_string(),
                avatar: "123".to_string(),
                metadata: None,
                is_owner: Some(true),
                is_bot: false,
                settings: None,
            },
            LeagueUser {
                username: Some("Someone".to_string()),
                user_id: "EVERYONE!".to_string(),
                display_name: "EVERYONE!".to_string(),
                avatar: "123".to_string(),
                metadata: None,
                is_owner: Some(true),
                is_bot: false,
                settings: None,
            },
            LeagueUser {
                username: Some("Someone".to_string()),
                user_id: "McJesus!".to_string(),
                display_name: "McJesus!".to_string(),
                avatar: "123".to_string(),
                metadata: None,
                is_owner: Some(true),
                is_bot: false,
                settings: None,
            },
        ];
        let expecteds = vec![
            SeasonPerformance {
                head_to_head_losses: 1,
                head_to_head_wins: 0,
                team_name: "Pete'sFarts".to_string(),
                league_wins: 1,
                league_losses: 0,
                points_against: 16.0,
                points_for: 15.0,
            },
            SeasonPerformance {
                head_to_head_losses: 0,
                head_to_head_wins: 1,
                team_name: "YOU!".to_string(),
                league_wins: 1,
                league_losses: 0,
                points_against: 15.0,
                points_for: 16.0,
            },
            SeasonPerformance {
                head_to_head_losses: 0,
                head_to_head_wins: 1,
                team_name: "EVERYONE!".to_string(),
                league_wins: 1,
                league_losses: 0,
                points_against: 6.0,
                points_for: 27.0,
            },
            SeasonPerformance {
                head_to_head_losses: 1,
                head_to_head_wins: 0,
                team_name: "McJesus!".to_string(),
                league_wins: 0,
                league_losses: 1,
                points_against: 27.0,
                points_for: 6.0,
            },
        ];
        for (idx, matchup) in matchups.iter().enumerate() {
            let actual = calculate_week_performance(matchup, &matchups, &rosters, &teams, 10.0);
            let expected = expecteds.get(idx).unwrap();
            assert_eq!(actual.head_to_head_losses, expected.head_to_head_losses);
            assert_eq!(actual.head_to_head_wins, expected.head_to_head_wins);
            assert_eq!(actual.league_losses, expected.league_losses);
            assert_eq!(actual.league_wins, expected.league_wins);
            assert_eq!(actual.points_against, expected.points_against);
            assert_eq!(actual.points_for, expected.points_for);
            assert_eq!(actual.team_name, expected.team_name);
        }
    }

    #[test]
    fn test_calc_ranks() {
        let inputs = vec![
            // Top
            SeasonPerformance {
                team_name: "Patrick".to_string(),
                head_to_head_losses: 2,
                head_to_head_wins: 12,
                league_wins: 8,
                league_losses: 6,
                points_for: 10.0,
                points_against: 10.0
            },
            // Bottom
            SeasonPerformance {
                team_name: "Hayden".to_string(),
                head_to_head_losses: 8,
                head_to_head_wins: 6,
                league_wins: 10,
                league_losses: 4,
                points_for: 10.0,
                points_against: 10.0
            },
            // Tiebreakers
            SeasonPerformance {
                team_name: "Beev".to_string(),
                head_to_head_losses: 7,
                head_to_head_wins: 7,
                league_wins: 10,
                league_losses: 4,
                points_for: 15.0,
                points_against: 10.0
            },
            SeasonPerformance {
                team_name: "Nabeel".to_string(),
                head_to_head_losses: 7,
                head_to_head_wins: 7,
                league_wins: 10,
                league_losses: 4,
                points_for: 14.0,
                points_against: 10.0
            }
        ];
        let output = vec![
            Standing {
                team_name: "Patrick".to_string(),
                rank_head_to_head: 0, // 0 indexed
                rank_league: 3,
                rank_combined: 0,
                season_performance: inputs.get(0).unwrap().clone()
            },
            Standing {
                team_name: "Hayden".to_string(),
                rank_head_to_head: 3, // last, 0 index == 4th
                rank_league: 2,
                rank_combined: 3,
                season_performance: inputs.get(1).unwrap().clone()
            },
            Standing {
                team_name: "Beev".to_string(),
                rank_head_to_head: 1,
                rank_league: 0,
                rank_combined: 1,
                season_performance: inputs.get(2).unwrap().clone()
            },
            Standing {
                team_name: "Nabeel".to_string(),
                rank_head_to_head: 2,
                rank_league: 1, // Tied with Beev, loses on points
                rank_combined: 2,
                season_performance: inputs.get(3).unwrap().clone()
            }
        ];
        let actual = calc_ranks(inputs);
        for i in output {
            for j in &actual {
                if i.team_name == j.team_name{
                    assert_eq!(&i, j);
                }
            }
        }
    }
}
