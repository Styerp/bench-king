use std::collections::HashMap;

use bench_king_sleeper::{client::SleeperClient, models::user::UserId};

const LEAGUE_ID: &str = "1124926301107884032";
const THROUGH_WEEK: i32 = 14;
#[derive(Clone)]
pub struct SeasonPerformance {
    team_name: String,
    head_to_head_wins: i8,
    head_to_head_losses: i8,
    league_wins: i8,
    league_losses: i8,
    points_for: f32,
    points_against: f32,
}

impl std::ops::Add for SeasonPerformance {
    type Output = SeasonPerformance;
    fn add(self, rhs: Self) -> Self::Output {
        SeasonPerformance {
            team_name: self.team_name.clone(),
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

#[derive(Clone)]
pub struct Standing {
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

async fn calculate_season_performances(league_id: String) -> Vec<SeasonPerformance> {
    let client = SleeperClient::build();
    let teams = client.get_users_in_league(league_id.clone()).await.unwrap();
    let rosters = client
        .get_rosters_in_league(league_id.clone())
        .await
        .unwrap();

    let mut data: HashMap<UserId, SeasonPerformance> = HashMap::new();
    for week in 1..(THROUGH_WEEK + 1) {
        let wk = client
            .get_league_matchups_for_week(league_id.clone(), week)
            .await
            .unwrap();
        let mut median_setup: Vec<f32> = wk
            .iter()
            .map(|m| m.starters_points.clone().iter().sum())
            .collect();
        median_setup.sort_by_key(|a| (a * 1000.0 )as i32);
        let median = (median_setup.get(6).unwrap() + median_setup.get(7).unwrap()) / 2.0;
        for matchup in &wk {
            let opp = &wk
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
            data.entry(owner.user_id.clone())
                .and_modify(|r| *r += wk_perf.clone())
                .or_insert(wk_perf.clone());
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
