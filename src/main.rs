
use bench_king_sleeper::client::SleeperClient;

const LEAGUE_ID: &str = "1124926301107884032";
const PATRICKS_USER_ID: &str = "1126996395593973760";

#[tokio::main]
async fn main() {
    let sleeper_client = SleeperClient::build();
    let resp = sleeper_client
        .get_users_in_league(LEAGUE_ID.to_string())
        .await
        .unwrap();
    println!("{:?}", resp);
}
