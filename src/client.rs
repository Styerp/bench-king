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
    player::{Player, TrendingPlayer},
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
    GetTrendingPlayers,
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
    TrendingPlayers(Vec<TrendingPlayer>),
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
    Invalid,
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
        match self.call_sleeper::<User>(url).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
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
        match self.call_sleeper::<Vec<League>>(url).await {
            Ok(leagues) => Ok(leagues),
            Err(e) => Err(e),
        }
    }

    pub async fn get_users_in_league(&self, league_id: String) -> Result<Vec<LeagueUser>, String> {
        let url = format!("{BASE_URL}league/{}/users", league_id);
        match self.call_sleeper::<Vec<LeagueUser>>(url).await {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }

    pub async fn get_rosters_in_league(&self, league_id: String) -> Result<Vec<Roster>, String> {
        let url = format!("{BASE_URL}league/{}/rosters", league_id);
        match self.call_sleeper::<Vec<Roster>>(url).await {
            Ok(rosters) => Ok(rosters),
            Err(e) => Err(e),
        }
    }

    pub async fn get_matchups_for_week(
        &self,
        league_id: String,
        week: String,
    ) -> Result<Vec<Matchup>, String> {
        let url = format!("{BASE_URL}league/{}/matchups/{}", league_id, week);
        match self.call_sleeper::<Vec<Matchup>>(url).await {
            Ok(matchups) => Ok(matchups),
            Err(e) => Err(e),
        }
    }

    pub async fn get_league_details(&self, league_id: String) -> Result<League, String> {
        let url = format!("{BASE_URL}league/{}", league_id);
        match self.call_sleeper::<League>(url).await {
            Ok(league) => Ok(league),
            Err(e) => Err(e),
        }
    }

    pub async fn get_avatar(
        &self,
        avatar_id: String,
        full_or_thumb: String,
    ) -> Result<String, String> {
        let url = format!("{BASE_URL}avatars/{}/{}", avatar_id, full_or_thumb);
        match self.call_sleeper::<String>(url).await {
            Ok(avatar) => Ok(avatar),
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_all_players(&self) -> Result<Vec<Player>, String> {
        let url = format!("{BASE_URL}players/nfl");
        match self.call_sleeper::<Vec<Player>>(url).await {
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
            ActionType::Invalid => return Err("Invalid action type".to_string()),
        };
        let url = format!("{BASE_URL}players/{sport}/trending/{type}?lookback_hours={lookback_hours}&limit={limit}", sport=sport.unwrap_or(Sport::default()), 
        type=action_type_str, lookback_hours=lookback_hours.unwrap_or("24".to_string()), limit=limit.unwrap_or("25".to_string()));
        match self.call_sleeper::<Vec<TrendingPlayer>>(url).await {
            Ok(players) => Ok(players),
            Err(e) => Err(e),
        }
    }

    // Generic function to call any API endpoint modeled.
    // Params are a hashmap of key value pairs to be used in the API call
    // Params are validated before calling the API
    pub async fn get_data(
        &self,
        api: SleeperApi,
        params: HashMap<String, String>,
    ) -> Result<SleeperResultType, String> {
        let mut missing_params: Vec<String> = Vec::new();
        match api {
            SleeperApi::GetUser => {
                let user_identifier = match params.get("user_id") {
                    Some(value) => value,
                    None => {
                        missing_params.push("user_id".to_string());
                        ""
                    }
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self.get_user(user_identifier.to_string()).await {
                    Ok(user) => Ok(SleeperResultType::User(user)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetAllLeaguesForUser => {
                let inputs = {
                    let user_id = match params.get("user_id") {
                        Some(value) => value,
                        None => {
                            missing_params.push("user_id".to_string());
                            ""
                        }
                    };
                    let season = match params.get("season") {
                        Some(value) => value,
                        None => {
                            missing_params.push("season".to_string());
                            ""
                        }
                    };
                    let sport = match params.get("sport") {
                        Some(value) => value,
                        None => "nfl",
                    };
                    (user_id, season, sport)
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self
                    .get_all_leagues_for_user(
                        inputs.0.to_string(),
                        inputs.1.to_string(),
                        Some(Sport(inputs.2.to_string())),
                    )
                    .await
                {
                    Ok(leagues) => Ok(SleeperResultType::Leagues(leagues)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetAvatar => {
                let inputs = {
                    let avatar_id = match params.get("avatar_id") {
                        Some(value) => value,
                        None => {
                            missing_params.push("avatar_id".to_string());
                            ""
                        }
                    };
                    let full_or_thumb = match params.get("full_or_thumb") {
                        Some(value) => value,
                        None => {
                            missing_params.push("full_or_thumb".to_string());
                            ""
                        }
                    };
                    (avatar_id, full_or_thumb)
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self
                    .get_avatar(inputs.0.to_string(), inputs.1.to_string())
                    .await
                {
                    Ok(avatar) => Ok(SleeperResultType::Avatar(avatar)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetLeagueDetails => {
                let league_id = match params.get("league_id") {
                    Some(value) => value,
                    None => {
                        missing_params.push("league_id".to_string());
                        ""
                    }
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self.get_league_details(league_id.to_string()).await {
                    Ok(league) => Ok(SleeperResultType::LeagueDetails(league)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetRostersInLeague => {
                let league_id = match params.get("league_id") {
                    Some(value) => value,
                    None => {
                        missing_params.push("league_id".to_string());
                        ""
                    }
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self.get_rosters_in_league(league_id.to_string()).await {
                    Ok(rosters) => Ok(SleeperResultType::Rosters(rosters)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetUsersInLeague => {
                let league_id = match params.get("league_id") {
                    Some(value) => value,
                    None => {
                        missing_params.push("league_id".to_string());
                        ""
                    }
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self.get_users_in_league(league_id.to_string()).await {
                    Ok(users) => Ok(SleeperResultType::LeagueUsers(users)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::GetMatchupsForWeek => {
                let inputs = {
                    let league_id = match params.get("league_id") {
                        Some(value) => value,
                        None => {
                            missing_params.push("league_id".to_string());
                            ""
                        }
                    };
                    let week = match params.get("week") {
                        Some(value) => value,
                        None => {
                            missing_params.push("week".to_string());
                            ""
                        }
                    };
                    (league_id, week)
                };
                if missing_params.len() > 0 {
                    return Err(format!("Missing required params: {:?}", missing_params));
                }
                match self
                    .get_matchups_for_week(inputs.0.to_string(), inputs.1.to_string())
                    .await
                {
                    Ok(matchups) => Ok(SleeperResultType::Matchups(matchups)),
                    Err(e) => Err(e),
                }
            }
            SleeperApi::FetchAllPlayers => match self.fetch_all_players().await {
                Ok(players) => Ok(SleeperResultType::Players(players)),
                Err(e) => Err(e),
            },
            SleeperApi::GetTrendingPlayers => {
                let inputs = {
                    let action_type = {
                        let temp = match params.get("action_type") {
                            Some(value) => value,
                            None => {
                                missing_params.push("action_type".to_string());
                                ""
                            }
                        };
                        match temp {
                            "add" => ActionType::Add,
                            "drop" => ActionType::Drop,
                            _ => {
                                missing_params.push("action_type".to_string());
                                ActionType::Invalid
                            }
                        }
                    };
                    let sport = match params.get("sport") {
                        Some(value) => Sport(value.to_string()),
                        None => Sport::default(),
                    };
                    let lookback_hours = match params.get("lookback_hours") {
                        Some(value) => value,
                        None => "24",
                    };
                    let limit = match params.get("limit") {
                        Some(value) => value,
                        None => "25",
                    };
                    (action_type, sport, lookback_hours, limit)
                };
                match self
                    .get_trending_players(
                        inputs.0,
                        Some(inputs.1),
                        Some(inputs.2.into()),
                        Some(inputs.3.into()),
                    )
                    .await
                {
                    Ok(players) => Ok(SleeperResultType::TrendingPlayers(players)),
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
