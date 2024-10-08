use std::collections::HashMap;

use bench_king_sleeper::calculation_helpers::calculate_bench_king_for_week::calculate_bench_king_for_week;
use bench_king_sleeper::calculation_helpers::report::Report;
use bench_king_sleeper::client::SleeperClient;
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
    week: i32,
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
    let rosters = sleeper_client
        .get_rosters_in_league(league_id.clone())
        .await
        .unwrap();
    let players = sleeper_client.fetch_all_players().await.unwrap();
    let league = sleeper_client
        .get_league_details(league_id.clone())
        .await
        .unwrap();
    let owners = sleeper_client
        .get_users_in_league(league_id.clone())
        .await
        .unwrap();
    let mut optimals ;
    if args.season_to_date {

        let mut week_map: HashMap<String, Vec<Report>> = HashMap::new();
        for week in 1..args.week {
            let matchups = sleeper_client
                .get_league_matchups_for_week(league_id.clone(), week)
                .await
                .unwrap();
            let optimals =
                calculate_bench_king_for_week(matchups, &rosters, &players, &league, &owners);
            for optimal in optimals {
                week_map
                    .entry(optimal.owner_name.clone()).and_modify(|f| f.push(optimal.clone()))
                    .or_insert(vec![optimal.clone()]);
            }
        }
        optimals = week_map.iter().map(|(owner, reports)| {
            let mut total_optimal_points = 0.0;
            let mut total_actual_points = 0.0;
            for report in reports {
                total_optimal_points += report.optimal_points;
                total_actual_points += report.actual_points;
            }
            Report {
                owner_name: owner.clone(),
                optimal_points: total_optimal_points,
                actual_points: total_actual_points,
            }
        }).collect();

    } else {
        let matchups = sleeper_client
                .get_league_matchups_for_week(league_id.clone(), args.week)
                .await
                .unwrap();
        optimals =
            calculate_bench_king_for_week(matchups, &rosters, &players, &league, &owners);
    }
    optimals.sort_by_key(|a| -1 * a.difference() as i32);
        for (idx, optimal) in optimals.iter().enumerate() {
            println!("Bench King Rank: {}: {}", idx + 1, optimal);
        }
    //println!("{:#?}", optimals);
}
