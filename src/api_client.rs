use reqwest::Client;

use crate::brewery::Brewery;

pub struct ApiService {
    pub client: Client,
    pub base_url: String,
}

impl ApiService {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_random_brewery(&self) -> Result<Brewery, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/breweries/random", self.base_url);
        let random_breweries = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<Brewery>>()
            .await?;

        let random_brewery = random_breweries
            .into_iter()
            .next()
            .ok_or("Empty response from /random endpoint")?;

        Ok(random_brewery)
    }
}
