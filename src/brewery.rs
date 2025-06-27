use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Brewery {
    pub name: String,
    pub id: Option<String>,
    pub brewery_type: String,
    pub address_1: Option<String>,
    pub address_2: Option<String>,
    pub address_3: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub state: String,
    pub street: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorLog {
    pub brewery: Brewery,
    pub reason: String,
}
