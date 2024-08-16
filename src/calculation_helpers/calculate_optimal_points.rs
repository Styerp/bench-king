use std::collections::HashMap;

use crate::models::{
    matchup::Matchup,
    player::{Players, PlayerDetails},
    positions::RosterPosition,
    roster::{Roster, RosterId},
};

pub struct OptimalScoreForMatchup {
    pub matchup_id: u16,
    pub roster_id: RosterId,
    pub owner_id: String,
    pub actual_points: f32,
    pub optimal_points: f32,
}

pub fn optimal_score_for_matchup(
    matchup: Matchup,
    roster: Roster,
    players: Players,
    league_positions: Vec<RosterPosition>,
) -> OptimalScoreForMatchup {
    let mut roster_position_count = league_positions
        .iter()
        .fold(HashMap::new(), |mut acc, pos| {
            if pos == &RosterPosition::BN {
                return acc;
            }
            let count: &mut u8 = acc.entry(pos).or_insert(0);
            *count += 1;
            acc
        })
        .iter()
        .map(|(pos, count)| (pos.clone().clone(), *count))
        .collect::<Vec<(RosterPosition, u8)>>();

    // Sort by the number of football positions a fantasy position can hold
    roster_position_count.sort_by_key(|a| a.0.value().len());

    let viable_players_with_stats = players
        .iter()
        .filter(|(player_id, _)| match &roster.players {
            Some(roster_players) => roster_players.iter().any(|rp| &rp == player_id),
            None => false,
        })
        .map(|(_, details)|details)
        .collect::<Vec<&PlayerDetails>>();

    let mut used_players: Vec<String> = Vec::new();
    let mut optimal_roster: OptimalScoreForMatchup = OptimalScoreForMatchup {
        matchup_id: matchup.matchup_id,
        roster_id: roster.roster_id,
        owner_id: roster.owner_id,
        actual_points: matchup.points,
        optimal_points: 0.0,
    };

    for (position, count) in roster_position_count {
        let mut players_for_position = viable_players_with_stats
            .iter()
            //.zip(&position)
            .filter(|&player| match &player.fantasy_positions {
                Some(fp) => fp.contains(&position),
                None => false,
            }
            )
            .collect::<Vec<&&PlayerDetails>>();

        players_for_position.sort_by_key(|a| {
            let a_points = *matchup.players_points.get(&a.player_id).unwrap() as i32;

            a_points * -1
        });

        for _ in 1..count {
            let player = match players_for_position.pop() {
                Some(p) => p,
                None => continue,
            };
            let player_id = &player.player_id;
            if used_players.contains(&player_id) {
                continue;
            } else {
                used_players.push(player_id.to_string());
                optimal_roster.optimal_points += matchup.players_points.get(player_id).unwrap();
            }
        }
    }
    optimal_roster
}
