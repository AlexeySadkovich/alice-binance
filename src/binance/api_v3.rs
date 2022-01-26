extern crate reqwest;

use crate::binance::models::TicketPriceData;

#[derive(Clone)]
pub struct APIv3 {
        token: String,
        client: reqwest::Client,
}

static URL: &str = "https://api.binance.com/api/v3";

impl APIv3 {
        pub fn new(token: String) -> APIv3 {
                let client = reqwest::Client::new();

                APIv3{ token, client }
        }

        pub async fn get_exchange_rate(&self, symbol: String) -> Result<f64, String> {
                let mut request = self.client.get(URL.to_owned() + "/ticker/price?symbol=" + symbol.as_str());

                let mut response = request.send().await.unwrap();
                let json = response.text().await.unwrap();

                let data = serde_json::from_str::<TicketPriceData>(&*json).unwrap();

                self.parse_price(data.price)
        }

        fn parse_price(&self, value: String) -> Result<f64, String> {
                match value.parse::<f64>() {
                        Ok(price) => Ok(price),
                        Err(_) => Result::Err(String::from("invalid price value"))
                }
        }
}