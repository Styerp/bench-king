use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// {
//     "starters": ["421", "4035", "3242", "2133", "2449", "4531", "2257", "788", "PHI"],
//     "roster_id": 1,
//     "players": ["1352", "1387", "2118", "2133", "2182", "223", "2319", "2449", "3208", "4035", "421", "4881", "4892", "788", "CLE"],
//     "matchup_id": 2,
//     "points": 20.0 // total points for team based on league settings
//     "custom_points": null // if commissioner overrides points manually
//   }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Matchup {
    pub starters: Vec<String>,
    pub roster_id: u32,
    pub players: Vec<String>,
    pub matchup_id: u16,
    pub points: f64,
    pub custom_points: Option<f64>,
    pub players_points: HashMap<String, f32>,
    pub starters_points: Vec<f32>,
}

impl Matchup {
    pub fn bench_players(&self) -> Vec<String> {
        self.players
            .clone()
            .into_iter()
            .filter(|player| !self.starters.contains(player))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bench_players() {
        let expected = vec!["123", "456"];

        let data = Matchup {
            starters: vec!["234".to_string(), "567".to_string(), "DET".to_string()],
            roster_id: 1,
            players: vec![
                "123".to_string(),
                "456".to_string(),
                "234".to_string(),
                "567".to_string(),
                "DET".to_string(),
            ],
            matchup_id: 1,
            points: 20.0,
            custom_points: None,
            players_points: HashMap::from([]),
            starters_points: vec![],
        };
        assert_eq!(data.bench_players(), expected)
    }
}
