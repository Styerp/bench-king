use std::{collections::HashMap, error::Error, time::Duration};

use http_cache_reqwest::{
    CACacheManager, Cache, CacheMode, CacheOptions, HttpCache, HttpCacheOptions,
};
use reqwest::{Client, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::Deserializer;

use crate::models::{
    league::League, matchup::Matchup, player::{Players, TrendingPlayer}, playoff::Playoff, roster::Roster, user::{LeagueUser, User}
};

const BASE_URL: &'static str = "https://api.sleeper.app/v1/";
pub struct SleeperClient {
    pub client: ClientWithMiddleware,
}

pub enum AvatarType {
    Full,
    Thumb,
}

struct NotImplementedError;
impl std::fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Not implemented yet")
    }
}
pub struct Sport(pub String);
impl Default for Sport {
    fn default() -> Self {
        Sport("nfl".to_string())
    }
}
impl std::fmt::Display for Sport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum ActionType {
    Add,
    Drop,
}

pub enum WinnerOrLoser {
    Winner,
    Loser,
}

impl SleeperClient {
    pub fn build() -> SleeperClient {
        // TODO: Figure out how to have a different cache policy for different endpoints
        let cache_options = CacheOptions {
            immutable_min_time_to_live: Duration::from_secs(86400),
            shared: true,
            ignore_cargo_cult: false,
            cache_heuristic: 0.1,
        };
        let cache = Cache(HttpCache {
            manager: CACacheManager::default(),
            mode: CacheMode::Default,
            options: HttpCacheOptions {
                cache_options: Some(cache_options),
                cache_key: Default::default(),
                cache_bust: Default::default(),
                cache_mode_fn: Default::default(),
            },
        });
        let client = ClientBuilder::new(Client::new()).with(cache).build();
        SleeperClient { client }
    }

    pub async fn get_user(&self, user_id: String) -> Result<User, String> {
        let url = format!("{BASE_URL}user/{}", user_id);
        match self.get_url(url).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    pub fn get_avatar_url(
        &self,
        avatar_id: String,
        full_or_thumb: AvatarType,
    ) -> String {
        let full_or_thumb = match full_or_thumb {
            AvatarType::Full => "",
            AvatarType::Thumb => "thumb",
        };
        format!("{BASE_URL}avatars/{}/{}", full_or_thumb, avatar_id)
    }

    pub async fn get_all_leagues_for_user(
        &self,
        user_id: String,
        season: String,
        sport: Option<Sport>,
    ) -> Result<Vec<League>, String> {
        let url = format!(
            "{BASE_URL}user/{}/leagues/{}/{}",
            user_id,
            sport.unwrap_or(Sport::default()),
            season
        );
        match self.get_url(url).await {
            Ok(leagues) => Ok(leagues),
            Err(e) => Err(e),
        }
    }

    pub async fn get_league_details(&self, league_id: String) -> Result<League, String> {
        let url = format!("{BASE_URL}league/{}", league_id);
        match self.get_url(url).await {
            Ok(league) => Ok(league),
            Err(e) => Err(e),
        }
    }

    pub async fn get_rosters_in_league(&self, league_id: String) -> Result<Vec<Roster>, String> {
        let url = format!("{BASE_URL}league/{}/rosters", league_id);
        match self.get_url(url).await {
            Ok(rosters) => Ok(rosters),
            Err(e) => Err(e),
        }
    }

    pub async fn get_users_in_league(&self, league_id: String) -> Result<Vec<LeagueUser>, String> {
        let url = format!("{BASE_URL}league/{}/users", league_id);
        match self.get_url(url).await {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }

    pub async fn get_league_matchups_for_week(
        &self,
        league_id: String,
        week: String,
    ) -> Result<Vec<Matchup>, String> {
        let url = format!("{BASE_URL}league/{}/matchups/{}", league_id, week);
        match self.get_url(url).await {
            Ok(matchups) => Ok(matchups),
            Err(e) => Err(e),
        }
    }

    // WIP: Need to figure out how deserialize t1_from/t2_from
    pub async fn get_playoff_bracket_for_league(
        &self,
        league_id: String,
        winner_or_loser: WinnerOrLoser,
    ) -> Result<Vec<Playoff>, String> {
        return Err(NotImplementedError.to_string());
        let url = match winner_or_loser {
            WinnerOrLoser::Winner => format!("{BASE_URL}league/{}/winners_bracket", league_id),
            WinnerOrLoser::Loser => format!("{BASE_URL}league/{}/losers_bracket", league_id),
        };
        match self.get_url(url).await {
            Ok(bracket) => Ok(bracket),
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_all_players(&self) -> Result<Players, String> {
        let url = format!("{BASE_URL}players/nfl");
        match self.get_url(url).await {
            Ok(players) => Ok(players),
            Err(e) => Err(e),
        }
    }

    pub async fn get_trending_players(
        &self,
        action_type: ActionType,
        sport: Option<Sport>,
        lookback_hours: Option<String>,
        limit: Option<String>,
    ) -> Result<Vec<TrendingPlayer>, String> {
        let action_type_str = match action_type {
            ActionType::Add => "add",
            ActionType::Drop => "drop",
        };
        let url = format!("{BASE_URL}players/{sport}/trending/{type}?lookback_hours={lookback_hours}&limit={limit}", sport=sport.unwrap_or(Sport::default()), 
        type=action_type_str, lookback_hours=lookback_hours.unwrap_or("24".to_string()), limit=limit.unwrap_or("25".to_string()));
        match self.get_url(url).await {
            Ok(players) => Ok(players),
            Err(e) => Err(e),
        }
    }

    /// Helper method to debug struct path errors
    async fn get_err_path<T>(&self, response: Response) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
    {
        let data = match response.text().await {
            Ok(text) => text,
            Err(e) => return Err(e.to_string()),
        };
        println!("{}", data);
        let deser = &mut Deserializer::from_str(data.as_str());
        let deser: Result<T, _> = serde_path_to_error::deserialize(deser);
        match deser {
            Ok(details) => Ok(details),
            Err(e) => {
                eprintln!("Error deserializing data: {}", e);
                Err(e.path().to_string())
            },
        }
    }

    // Generic callable
    async fn get_url<T>(&self, url: String) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<T>(response).await {
                Ok(data) => Ok(data),
                Err(e) => {
                    eprintln!("Error getting data: {}", e);
                    Err(e)
                }
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
