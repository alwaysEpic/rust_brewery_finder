use rust_brewery_finder::brewery::{Brewery, ErrorLog};
use rust_brewery_finder::{api_client::ApiService, brewery};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = ApiService::new("https://api.openbrewerydb.org");

    let mut seen_states = HashSet::new();
    let mut unique_state_breweries = Vec::new();
    let mut missed_criteria: Vec<(Brewery, String)> = Vec::new();

    while seen_states.len() < 5 {
        let rand_brew = api.get_random_brewery().await?;
        if rand_brew.brewery_type != "micro" {
            missed_criteria.push((rand_brew, "Not a micro brewery".to_string()));
        } else if !seen_states.insert(rand_brew.state.clone()) {
            missed_criteria.push((rand_brew, "State previously seen".to_string()));
        } else {
            unique_state_breweries.push(rand_brew);
        }
    }

    // for breweries in unique_state_breweries {
    //     println!("{}", breweries.state);
    // }
    let _ = write_json_to_file(
        &unique_state_breweries.clone(),
        "./logs/travel_destinations.json",
    );

    let error_logs: Vec<ErrorLog> = missed_criteria
        .into_iter()
        .map(|(brewery, reason)| ErrorLog { brewery, reason })
        .collect();
    let _ = write_json_to_file(&error_logs, "./logs/error_log.json");

    Ok(())
}

fn write_json_to_file<T: serde::Serialize>(data: &T, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data).expect("Serialization failed");
    std::fs::write(path, json)?;
    Ok(())
}
