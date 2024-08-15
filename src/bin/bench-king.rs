use bench_king_sleeper::calculation_helpers::calculate_optimal_points::optimal_roster_for_matchup;
use bench_king_sleeper::client::{SleeperClient, Sport};
use bench_king_sleeper::models::matchup;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, env = "SLEEPER_LEAGUE_ID", long_help = "The Sleeper league ID")]
    league_id: String,
    // #[arg(long, env = "SLEEPER_USER_ID", long_help = "The Sleeper user ID")]
    // user_id: Option<String>,
    #[arg(
        long,
        long_help = "Week to calculate bench king for",
        default_value = "1"
    )]
    week: String,
    #[arg(
        long,
        long_help = "Season to calculate bench king for",
        default_value = "2024"
    )]
    season: String,
    #[arg(
        long,
        long_help = "Sport to calculate bench king for",
        short,
        default_value = "nfl"
    )]
    sport: String,
    #[arg(
        long,
        long_help = "Flag to calculate season to date",
        default_value = "false"
    )]
    season_to_date: bool,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    let league_id = args.league_id;
    let sleeper_client = SleeperClient::build();
    let matchups = sleeper_client
        .get_matchups_for_week(league_id.clone(), args.week)
        .await
        .unwrap();
    let rosters = sleeper_client
        .get_rosters_in_league(league_id.clone())
        .await
        .unwrap();
    let players = sleeper_client.fetch_all_players().await.unwrap();
    let league = sleeper_client.get_league_details(league_id.clone()).await.unwrap();
    let owners = sleeper_client.get_users_in_league(league_id.clone()).await.unwrap();

    let mut optimals = vec![];
    for matchup in matchups {
        let roster = rosters
            .iter()
            .find(|r| r.roster_id == matchup.roster_id)
            .unwrap();
        let optimal_roster = optimal_roster_for_matchup(
            matchup.clone(),
            roster.clone(),
            players.clone(),
            league.roster_positions.clone(),
        );
        let rep = Report {
            owner_name: owners
                .iter()
                .find(|o| o.user_id == optimal_roster.owner_id)
                .unwrap()
                .display_name
                .clone(),
            optimal_points: optimal_roster.optimal_points,
            actual_points: optimal_roster.actual_points,
        };
        optimals.push(rep);
    }
    optimals.sort_by_key(|a| a.difference() as i32);
    println!("{:?}", optimals);
}

#[derive(Debug)]
pub struct Report {
    pub owner_name: String,
    pub optimal_points: f32,
    pub actual_points: f32,
}
impl Report {
    pub fn difference(&self) -> f32 {
        self.optimal_points - self.actual_points
    }
}
impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} - {} = {}",
            self.owner_name,
            self.optimal_points,
            self.actual_points,
            self.difference()
        )
    }
}