use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client, Error, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::Deserializer;
use serde_path_to_error::deserialize;

use crate::models::{league::League, matchup::Matchup, roster::Roster, user::{LeagueUser, User}};

const BASE_URL: &'static str = "https://api.sleeper.app/v1/";
pub struct SleeperClient {
    pub client: ClientWithMiddleware,
}
impl SleeperClient {
    pub fn build() -> SleeperClient {
        let cache = Cache(HttpCache {
            manager: CACacheManager::default(),
            mode: CacheMode::Default,
            options: HttpCacheOptions::default(),
        });
        let client = ClientBuilder::new(Client::new()).with(cache).build();
        SleeperClient { client }
    }

    pub async fn get_user(&self, user_id_or_name: String) -> Result<User, String> {
        let url = format!("{BASE_URL}user/{}", user_id_or_name);
        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<User>(response).await {
                Ok(user) => Ok(user),
                Err(e) => {
                    eprintln!("Error getting user: {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                eprintln!("Error getting user: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub async fn get_avatar(
        &self,
        avatar_id: String,
        full_or_thumb: Option<String>,
    ) -> Result<String, String> {
        let extension = match full_or_thumb {
            Some(value) => value,
            None => "".to_string(),
        };
        let url = format!("{BASE_URL}avatars/{}/{}", extension, avatar_id);
        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<String>(response).await {
                Ok(user) => Ok(user),
                Err(e) => {
                    eprintln!("Error getting avatar: {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                eprintln!("Error getting avatar: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub async fn get_all_leagues_for_user(
        &self,
        user_id: String,
        season: String,
        sport: Option<String>,
    ) -> Result<Vec<League>, String> {
        let sport = match sport {
            Some(value) => value,
            None => "nfl".to_string(),
        };
        let url = format!("{BASE_URL}user/{}/leagues/{}/{}", user_id, sport, season);
        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<Vec<League>>(response).await {
                Ok(leagues) => Ok(leagues),
                Err(e) => {
                    eprintln!("Error getting leagues for user. {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                eprintln!("Error getting leagues for user. {}", e);
                Err(e.to_string())
            }
        }
    }

    pub async fn get_league_details(&self, league_id: String) -> Result<League, String> {
        let url = format!("{BASE_URL}league/{}", league_id);

        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<League>(response).await {
                Ok(league) => Ok(league),
                Err(e) => {
                    eprintln!("Error getting league details: {}", e);
                    Err(e)
                },
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_rosters_in_league(&self, league_id: String) -> Result<Vec<Roster>, String> {
        let url = format!("{BASE_URL}league/{}/rosters", league_id);

        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<Vec<Roster>>(response).await {
                Ok(rosters) => Ok(rosters),
                Err(e) => {
                    eprintln!("Error getting rosters in league: {}", e);
                    Err(e)
                },
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_users_in_league(&self, league_id: String) -> Result<Vec<LeagueUser>, String> {
        let url = format!("{BASE_URL}league/{}/users", league_id);

        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<Vec<LeagueUser>>(response).await {
                Ok(users) => Ok(users),
                Err(e) => {
                    eprintln!("Error getting users in league: {}", e);
                    Err(e)
                },
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_matchups_for_week(&self, league_id: String, week: u32) -> Result<Vec<Matchup>, String> {
        let url = format!("{BASE_URL}league/{}/matchups/{}", league_id, week);

        match self.client.get(&url).send().await {
            Ok(response) => match self.get_err_path::<Vec<Matchup>>(response).await {
                Ok(rosters) => Ok(rosters),
                Err(e) => {
                    eprintln!("Error getting matchups for week: {}", e);
                    Err(e)
                },
            },
            Err(e) => Err(e.to_string()),
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
        let deser: Result<T, _> = deserialize(deser);
        match deser {
            Ok(details) => Ok(details),
            Err(e) => Err(e.path().to_string()),
        }
    }
}
