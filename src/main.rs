use bench_king_sleeper::client::{self, SleeperClient};

const LEAGUE_ID: &str = "1124926301107884032";
const PATRICKS_USER_ID: &str = "1126996395593973760";
const HAYDEN_LEAGUE_ID: &str = "1004906699078828032";
const SCUM_AND_VILLIANY: &str = "1124835525820166144";

#[tokio::main]
async fn main() {
    let sleeper_client = SleeperClient::build();
    let league_id = LEAGUE_ID.to_string();

    let resp = sleeper_client
        .get_league_matchups_for_week(&SCUM_AND_VILLIANY.to_string(), 14)
        .await
        .unwrap();
    let roster = resp.iter().find(|a| a.roster_id == 7).unwrap();
    let players = sleeper_client
        .fetch_all_players()
        .await
        .unwrap()
        .into_iter()
        .filter(|p| {
            roster.players.contains(&p.1.player_id)
        });
    // let league_users = sleeper_client
    //     .get_users_in_league(LEAGUE_ID.to_string())
    //     .await
    //     .unwrap();
    // let rosters = sleeper_client
    //     .get_playoff_bracket_for_league(LEAGUE_ID.to_string(), client::WinnerOrLoser::Winner)
    //     .await
    //     .unwrap();
    // let details = sleeper_client
    //     .get_league_details(LEAGUE_ID.to_string())
    //     .await
    //     .unwrap();
    // for user in league_users {
    //     // sleeper_client.get_user(user.user_id.clone()).await.unwrap();
    //     // sleeper_client.get_avatar(user.avatar, client::AvatarType::Full);
    //     sleeper_client.get_all_leagues_for_user(user.user_id, "2024".to_string(), None).await.unwrap();
    // }
    for player in players {
        println!("Id: {}, Name: {}, Postions: {:?}, Points: {:?}", &player.1.player_id, &player.1.full_name.unwrap_or_default(), &player.1.fantasy_positions, 
roster.players_points.get(&player.1.player_id).unwrap())
    }
}
