use std::collections::HashMap;

use crate::models::{
    matchup::Matchup,
    player::{PlayerDetails, Players},
    positions::RosterPosition,
    roster::{Roster, RosterId},
};

#[derive(Debug)]
pub struct OptimalScoreForMatchup {
    pub matchup_id: u16,
    pub roster_id: RosterId,
    pub owner_id: String,
    pub actual_points: f32,
    pub optimal_points: f32,
}

impl std::fmt::Display for OptimalScoreForMatchup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matchup ID: {}, Roster ID: {}, Owner ID: {}, Actual Points: {}, Optimal Points: {}",
            self.matchup_id, self.roster_id, self.owner_id, self.actual_points, self.optimal_points
        )
    }
}

pub fn optimal_score_for_matchup(
    matchup: Matchup,
    roster: Roster,
    players: Players,
    league_positions: Vec<RosterPosition>,
) -> OptimalScoreForMatchup {
    //println!("Matchup Points: {:?}", matchup.points);
    let mut optimal_roster = OptimalScoreForMatchup {
        matchup_id: matchup.matchup_id,
        roster_id: roster.roster_id,
        owner_id: roster.owner_id,
        actual_points: matchup.points,
        optimal_points: 0.0,
    };
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
        .map(|(pos, count)| (**pos, *count))
        .collect::<Vec<(RosterPosition, u8)>>();

    // Sort by the number of football positions a fantasy position can hold
    roster_position_count.sort_by_key(|a| (a.0.value().len(), a.1));
    //println!("Roster Position Count: {:?}", roster_position_count);

    let viable_players_with_stats = players
        .iter()
        .filter(|(player_id, _)| matchup.players.iter().any(|p| &p == player_id))
        .map(|(_, details)| details)
        .collect::<Vec<&PlayerDetails>>();
    //println!("Viable Players: {:?}", viable_players_with_stats.iter().map(|p| (p.full_name.as_ref().unwrap_or(&p.player_id), p.position.unwrap_or(RosterPosition::SuperFlex))).collect::<Vec<(&String, RosterPosition)>>());

    let mut used_players = Vec::new();
    
    for (position, count) in roster_position_count {
        //println!("Drafting {} players for position {:?}", count, position);
        let mut players_for_position = viable_players_with_stats
            .iter()
            .filter(|&player| match &player.fantasy_positions {
                Some(fp) => {
                    fp.iter().any(|p| position.value().iter().any(|v| p.value().contains(v)))
                },
                None => false,
            })
            .map(|p| p.to_owned().to_owned())
            .collect::<Vec<PlayerDetails>>();
        players_for_position.sort_by(|a, b| {
            let a_pts = matchup.players_points.get(&a.player_id).unwrap_or(&0.0f32);
            let b_pts = matchup.players_points.get(&b.player_id).unwrap_or(&0.0f32);
            a_pts.partial_cmp(b_pts).unwrap()
        });
        // println!("Players for position: {:?}", players_for_position.iter().map(|p| match &p.full_name {
        //     Some(name) => (name.to_string(), matchup.players_points.get(&p.player_id).unwrap_or(&0.0f32)),
        //     None => (p.player_id.to_string(), matchup.players_points.get(&p.player_id).unwrap_or(&0.0f32)),
        // }).collect::<Vec<(String,&f32)>>());
        for _l in 0..count {
            //println!("Looop count: {}, Player Count: {}, Position: {:?}", l, players_for_position.len(), position.value());
            let (player, up, pfp) = get_player(&used_players, players_for_position);
            used_players = [used_players, up].concat().clone();
            players_for_position = pfp;
            let player_id = &player.player_id;
            let points = matchup.players_points.get(player_id).unwrap_or(&0.0f32);
            //println!("Player: {:?}; Points: {}", player.full_name.as_ref().unwrap_or(&player.player_id), points);
            optimal_roster.optimal_points += points;
            used_players.push(player_id.to_string());
        }
    }
    optimal_roster
}

fn get_player(used_players: &Vec<String>, mut players: Vec<PlayerDetails>) -> (PlayerDetails, Vec<String>, Vec<PlayerDetails>) {
    match players.pop() {
        Some(p) => {
            if used_players.contains(&p.player_id) {
                //println!("Player already used: {:?}", p.full_name.as_ref().unwrap_or(&p.player_id));
                get_player(&[used_players.clone(), vec![p.player_id.clone()]].concat(), players.clone());
            }
            (p.clone().clone(), [used_players.clone(), vec![p.player_id.clone()]].concat(), players)
        },
        None => {
            get_player(used_players, players)
        },
    }
}