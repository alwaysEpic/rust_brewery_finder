# Brewery Finder Project
 
Feel like a road trip, but don't know where to go? Just run this app to find 5 mirco brewery's in 5 states! Now all you need is a vehicle and a friend!

---

## Project Overview

The project contains:

- **Main**  
  Finds you, your destinations 

- **Brewery Models**  
  Rust structs generated from random brewery API
  https://www.openbrewerydb.org/documentation

- **API Client**  
  Handles fetching rover configuration, exercises, and submitting computed results via `reqwest`.

---

## Dependencies

- [Tokio](https://crates.io/crates/tokio) (async runtime)
- [Reqwest](https://crates.io/crates/reqwest) (HTTP client)
- [Serde](https://crates.io/crates/serde) (Serialization/Deserialization)

---

## How It Works

**Run the application to retrieve your travel destinations and understand which breweries were rejected and why.**

After execution, two files will be generated in the `logs/` directory:

- `travel_destinations.json` – Contains **5 microbreweries**, each from a **unique U.S. state**.
- `error_log.json` – Lists all breweries that were rejected, along with the **reason** they didn’t meet the criteria (e.g., wrong type or duplicate state).

These logs provide both curated results and full transparency into the selection process.

---

## Example Usage

- 'cargo run' will run the application
- If you are using mscode, you can run in a dev container to avoid dependency on local machine
- From the rust_brewery_folder, right click in lower left corner on brackets and select "Reopen in Container"