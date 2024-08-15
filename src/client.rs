use std::{collections::HashMap, time::Duration};

use http_cache_reqwest::{
    CACacheManager, Cache, CacheMode, CacheOptions, HttpCache, HttpCacheOptions,
};
use http_cache_semantics::{CachePolicy, RequestLike};
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::Deserializer;

use crate::models::{
    league::League,
    matchup::Matchup,
    player::Player,
    roster::Roster,
    user::{LeagueUser, User},
};

const BASE_URL: &'static str = "https://api.sleeper.app/v1/";
pub struct SleeperClient {
    pub client: ClientWithMiddleware,
}
pub enum SleeperApi {
    GetUser,
    GetAvatar,
    GetAllLeaguesForUser,
    GetLeagueDetails,
    GetRostersInLeague,
    GetUsersInLeague,
    GetMatchupsForWeek,
    FetchAllPlayers,
}
#[derive(Debug)]
pub enum SleeperResultType {
    User(User),
    Avatar(String),
    Leagues(Vec<League>),
    LeagueDetails(League),
    Rosters(Vec<Roster>),
    LeagueUsers(Vec<LeagueUser>),
    Matchups(Vec<Matchup>),
    Players(Vec<Player>),
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

    pub async fn get_data(
        &self,
        api: SleeperApi,
        params: HashMap<String, String>,
    ) -> Result<SleeperResultType, String> {
        match api {
            SleeperApi::GetUser => {
                let user_identifier = match params.get("user_id") {
                    Some(value) => value,
                    None => return Err("No value for key user_id provided".to_string()),
                };
                let url = format!("{BASE_URL}user/{}", user_identifier);
                match self.call_sleeper::<User>(url).await {
                    Ok(user) => Ok(SleeperResultType::User(user)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetAllLeaguesForUser => {
                let inputs = {
                    let user_id = match params.get("user_id") {
                        Some(value) => value,
                        None => return Err("No value for key user_id provided".to_string()),
                    };
                    let season = match params.get("season") {
                        Some(value) => value,
                        None => return Err("No value for key season provided".to_string()),
                    };
                    let sport = match params.get("sport") {
                        Some(value) => value,
                        None => "nfl",
                    };
                    (user_id, season, sport)
                };
                let url = format!(
                    "{BASE_URL}user/{}/leagues/{}/{}",
                    inputs.0, inputs.2, inputs.1
                );
                match self.call_sleeper::<Vec<League>>(url).await {
                    Ok(leagues) => Ok(SleeperResultType::Leagues(leagues)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetAvatar => {
                let inputs = {
                    let avatar_id = match params.get("avatar_id") {
                        Some(value) => value,
                        None => return Err("No value for key avatar_id provided".to_string()),
                    };
                    let full_or_thumb = match params.get("full_or_thumb") {
                        Some(value) => value,
                        None => return Err("No value for key full_or_thumb provided".to_string()),
                    };
                    (avatar_id, full_or_thumb)
                };
                let url = format!("{BASE_URL}avatars/{}/{}", inputs.0, inputs.1);
                match self.call_sleeper::<String>(url).await {
                    Ok(avatar) => Ok(SleeperResultType::Avatar(avatar)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetLeagueDetails => {
                let league_id = match params.get("league_id") {
                    Some(value) => value,
                    None => return Err("No value for key league_id provided".to_string()),
                };
                let url = format!("{BASE_URL}league/{}", league_id);
                match self.call_sleeper::<League>(url).await {
                    Ok(league) => Ok(SleeperResultType::LeagueDetails(league)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetRostersInLeague => {
                let league_id = match params.get("league_id") {
                    Some(value) => value,
                    None => return Err("No value for key league_id provided".to_string()),
                };
                let url = format!("{BASE_URL}league/{}/rosters", league_id);
                match self.call_sleeper::<Vec<Roster>>(url).await {
                    Ok(rosters) => Ok(SleeperResultType::Rosters(rosters)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetUsersInLeague => {
                let league_id = match params.get("league_id") {
                    Some(value) => value,
                    None => return Err("No value for key league_id provided".to_string()),
                };
                let url = format!("{BASE_URL}league/{}/users", league_id);
                match self.call_sleeper::<Vec<LeagueUser>>(url).await {
                    Ok(users) => Ok(SleeperResultType::LeagueUsers(users)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetMatchupsForWeek => {
                let inputs = {
                    let league_id = match params.get("league_id") {
                        Some(value) => value,
                        None => return Err("No value for key league_id provided".to_string()),
                    };
                    let week = match params.get("week") {
                        Some(value) => value,
                        None => return Err("No value for key week provided".to_string()),
                    };
                    (league_id, week)
                };
                let url = format!("{BASE_URL}league/{}/matchups/{}", inputs.0, inputs.1);
                match self.call_sleeper::<Vec<Matchup>>(url).await {
                    Ok(matchups) => Ok(SleeperResultType::Matchups(matchups)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::FetchAllPlayers => {
                let url = format!("{BASE_URL}players/nfl");
                match self.call_sleeper::<Vec<Player>>(url).await {
                    Ok(players) => Ok(SleeperResultType::Players(players)),
                    Err(e) => Err(e),
                }
            }
        }
    }

    // Helper method to debug struct path errors
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
            Err(e) => Err(e.path().to_string()),
        }
    }

    // Generic callable
    async fn call_sleeper<T>(&self, url: String) -> Result<T, String>
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
