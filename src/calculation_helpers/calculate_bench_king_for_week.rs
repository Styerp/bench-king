use super::{calculate_optimal_points::optimal_score_for_matchup, report::Report};
use crate::models::*;

pub fn calculate_bench_king_for_week(
    matchups: Vec<matchup::Matchup>,
    rosters: &Vec<roster::Roster>,
    players: &player::Players,
    league: &league::League,
    owners: &Vec<user::LeagueUser>,
) -> Vec<Report> {
    let mut optimals = vec![];
    for matchup in matchups {
        let roster = rosters
            .iter()
            .find(|r| r.roster_id == matchup.roster_id)
            .unwrap();
        //println!("======== Owner: {:?} ========", owners.clone().into_iter().filter(|f| f.user_id == roster.owner_id).map(|m| m.display_name).collect::<Vec<_>>());
        let optimal_roster = optimal_score_for_matchup(
            matchup.clone(),
            roster.clone(),
            players.clone(),
            league.roster_positions.clone(),
        );

        let rep = Report {
            owner_name: owners
                .iter()
                .find(|o| o.user_id == optimal_roster.owner_id)
                .unwrap()
                .display_name
                .clone(),
            optimal_points: optimal_roster.optimal_points,
            actual_points: optimal_roster.actual_points,
        };
        optimals.push(rep);
    }
    optimals
}
