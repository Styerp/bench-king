use std::collections::{HashMap, HashSet};

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
            acc.entry(pos).and_modify(|c| *c += 1).or_insert(1);
            acc
        })
        .iter()
        .map(|(pos, count)| (**pos, *count))
        .collect::<Vec<(RosterPosition, u8)>>();

    // Sort by the number of football positions a fantasy position can hold, fewest first.
    // When slotting players, the highest point player should go into the most restrictive slot, because
    // that allow flex spot to use more positions. In the case of a tie, put them in the one with the most
    // slots, to allow a more restricted player to fit into the other slot.
    roster_position_count.sort_by_key(|a| (a.0.value().len(), a.1 as i32 * -1));

    let viable_players_with_stats = players
        .iter()
        .filter(|(player_id, _)| matchup.players.iter().any(|p| &p == player_id))
        .map(|(_, details)| details)
        .collect::<Vec<&PlayerDetails>>();

    let mut used_players = HashSet::new();

    for (position, count) in roster_position_count {
        // Any position the player plays is associated with any position the league slots.
        let mut players_for_position = viable_players_with_stats
            .iter()
            .filter(|&player| match &player.fantasy_positions {
                Some(fp) => fp.iter().any(|p| {
                    position.value().iter().any(|v| {
                        p.value().contains(v) && matchup.players.contains(&player.player_id)
                    })
                }),
                None => false,
            })
            .map(|p| p.to_owned().to_owned())
            .collect::<Vec<PlayerDetails>>();
        // Sort the vec by points, highest _last_ so we can pop the players off later.
        players_for_position.sort_by_key(|a| {
            (matchup.players_points.get(&a.player_id).unwrap_or(&0.0f32) * 1000.0) as i32
        });

        for _l in 0..count {
            if players_for_position.len() == 0 {
                continue;
            }
            let (player, up, pfp) = get_player(&mut used_players, players_for_position);
            used_players = up.clone();
            players_for_position = pfp;
            let points = matchup
                .players_points
                .get(&player.player_id)
                .unwrap_or(&0.0f32);
            optimal_roster.optimal_points += points;
            used_players.insert(player.player_id.to_string());
        }
    }
    optimal_roster
}

fn get_player(
    used_players: &mut HashSet<String>,
    mut players: Vec<PlayerDetails>,
) -> (PlayerDetails, &HashSet<String>, Vec<PlayerDetails>) {
    // Take the last player. We sort ASC on the ppints, so this is the highest point player remaining.
    match players.pop() {
        Some(p) => {
            let is_used = used_players.contains(&p.player_id);
            used_players.insert(p.player_id.clone());
            if is_used {
                return get_player(used_players, players);
            }
            (p.clone().clone(), used_players, players)
        }
        None => get_player(used_players, players),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_player() {
        let mut players = vec![PlayerDetails::default(
            true,
            "1".to_string(),
            "nfl".to_string(),
        ),
        PlayerDetails::default(
            true,
            "2".to_string(),
            "nfl".to_string(),
        ),PlayerDetails::default(
            true,
            "3".to_string(),
            "nfl".to_string(),
        ),PlayerDetails::default(
            true,
            "4".to_string(),
            "nfl".to_string(),
        ),
        PlayerDetails::default(
            true,
            "4".to_string(),
            "nfl".to_string(),
        ),
        ];
        let mut used_players: HashSet<String> = HashSet::new();

        let mut expected_used_players = HashSet::new();
        expected_used_players.insert("4".to_string());
        let actual = get_player(&mut used_players, players);
        let expected = (PlayerDetails::default(
            true,
            "4".to_string(),
            "nfl".to_string(),
        ),&expected_used_players ,
        vec![PlayerDetails::default(
            true,
            "1".to_string(),
            "nfl".to_string(),
        ),
        PlayerDetails::default(
            true,
            "2".to_string(),
            "nfl".to_string(),
        ),PlayerDetails::default(
            true,
            "3".to_string(),
            "nfl".to_string(),
        ),PlayerDetails::default(
            true,
            "4".to_string(),
            "nfl".to_string(),
        ),]);
        assert_eq!(actual, expected);
        players = vec![PlayerDetails::default(
            true,
            "1".to_string(),
            "nfl".to_string(),
        ),
        PlayerDetails::default(
            true,
            "2".to_string(),
            "nfl".to_string(),
        ),PlayerDetails::default(
            true,
            "3".to_string(),
            "nfl".to_string(),
        ),PlayerDetails::default(
            true,
            "4".to_string(),
            "nfl".to_string(),
        ),];
        let second_actual = get_player(&mut used_players, players);
        expected_used_players.insert("3".to_string());
        let second_expected = (
            PlayerDetails::default(
                true,
                "3".to_string(),
                "nfl".to_string(),
            ),
            &expected_used_players,
            vec![PlayerDetails::default(
                true,
                "1".to_string(),
                "nfl".to_string(),
            ),
            PlayerDetails::default(
                true,
                "2".to_string(),
                "nfl".to_string(),
            )]
        );
        assert_eq!(second_actual,second_expected);

    }
}
