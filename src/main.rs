use rust_brewery_finder::api_client::ApiService;
use rust_brewery_finder::brewery::{Brewery, ErrorLog};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = ApiService::new("https://api.openbrewerydb.org");

    let mut seen_states = HashSet::new();
    let mut unique_state_breweries = Vec::new();
    let mut missed_criteria: Vec<(Brewery, String)> = Vec::new();

    // Loop until we get 5 mirco breweries in different states, tracking rejects
    while seen_states.len() < 5 {
        let rand_brew = api.get_random_brewery().await?;
        // broke up mirco/state check to note different reject reasons
        if rand_brew.brewery_type != "micro" {
            missed_criteria.push((rand_brew, "Not a micro brewery".to_string()));
            continue;
        }
        if !seen_states.insert(rand_brew.state.clone()) {
            missed_criteria.push((rand_brew, "State previously seen".to_string()));
            continue;
        }

        unique_state_breweries.push(rand_brew);
    }

    //save unique and rejects to file
    write_json_to_file(&unique_state_breweries, "./logs/travel_destinations.json");

    let error_logs: Vec<ErrorLog> = missed_criteria
        .into_iter()
        .map(|(brewery, reason)| ErrorLog { brewery, reason })
        .collect();
    write_json_to_file(&error_logs, "./logs/error_log.json");

    println!("Success: Found 5 mirco breweries in 5 states, details in log folder");
    Ok(())
}

// serializes and writes data to file
fn write_json_to_file<T: serde::Serialize>(data: &T, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data).expect("Serialization failed");
    std::fs::write(path, json)
}
