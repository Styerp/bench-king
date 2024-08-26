use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BracketProgression {
    RosterId(u8),
    Winner { w: u8 },
    Loser { l: u8 },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playoff {
    #[serde(alias = "r")]
    pub round: u8,
    #[serde(alias = "m")]
    pub matchup_id: u8,
    /// The roster_id of a team in this matchup OR {w: 1} which means the winner of match id 1
    #[serde(alias = "t1")] //, deserialize_with = "u8_or_struct")]
    pub roster_id_1_or_winner_matchup: Option<BracketProgression>,
    /// The roster_id of the other team in this matchup OR {l: 1} which means the loser of match id 1
    #[serde(alias = "t2")] //, deserialize_with = "u8_or_struct")]
    pub roster_id_2_or_loser_of_matchup_id: Option<BracketProgression>,
    #[serde(alias = "w")]
    pub winner: Option<u8>,
    #[serde(alias = "l")]
    pub loser: Option<u8>,
    pub t1_from: Option<BracketProgression>,
    pub t2_from: Option<BracketProgression>,
}

impl Display for Playoff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Playoff: {}", self)
    }
}

// impl From<u8> for BracketProgression {
//     fn from(v: u8) -> Self {
//         BracketProgression::RosterId(v)
//     }
// }

// fn u8_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
// where
//     T: Deserialize<'de> + From<u8>,
//     D: Deserializer<'de>,
// {
//     struct StringOrStruct<T>(PhantomData<fn() -> T>);

//     impl <'de, T> Visitor<'de> for StringOrStruct<T>
//     where
//         T: Deserialize<'de> + From<u8>
//         {
//             type Value = T;
//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("a u8 or a struct")
//             }
//             fn visit_u8<E>(self, v: u8) -> Result<T, E>
//                 where
//                     E: Error,
//             {
//                 Ok(v.into())
//             }
//             fn visit_map<A>(self, map: A) -> Result<T, A::Error>
//                 where
//                     A: MapAccess<'de>,
//             {
//                 Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
//             }
//         }

//     deserializer.deserialize_any(StringOrStruct(PhantomData))
// }
