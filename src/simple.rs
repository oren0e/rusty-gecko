use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::SimpleResponseError;
use crate::generic::GeckoRequest;

pub type Price = f32;
pub type Coin = String;
pub type Currency = String;
pub type SimplePrice = HashMap<Currency, Price>;
pub type SimplePrices = HashMap<Coin, SimplePrice>;

#[derive(Debug, Deserialize, Default)]
pub struct SimplePriceRequest {
    pub ids: String,
    pub vs_currencies: String,
    pub include_market_cap: bool,
    pub include_24hr_vol: bool,
    pub include_24hr_change: bool,
    pub include_last_updated_at: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimplePriceResponse {
    pub simple_response: SimplePrices,
}

impl SimplePriceResponse {
    pub fn validate_response(
        &self,
        coin_ids: &[&str],
        currencies: &[&str],
    ) -> Result<SimplePrices, SimpleResponseError> {
        for coin in coin_ids {
            if !&self.simple_response.contains_key(&coin.to_string()) {
                return Err(SimpleResponseError::UnknownCoinError(coin.to_string()));
            }
            for currency in currencies {
                if &self
                    .simple_response
                    .get(&coin.to_string())
                    .map(|x| x.contains_key(&currency.to_string()))
                    != &Some(true)
                {
                    return Err(SimpleResponseError::UnknownCurrencyError(
                        currency.to_string(),
                    ));
                }
            }
        }
        return Ok(self.simple_response.clone());
    }
}

impl GeckoRequest for SimplePriceRequest {
    fn get_json<T: DeserializeOwned>(&self) -> Result<T, SimpleResponseError> {
        let response: T = Client::new()
            .get(format!(
                "{}{}{}",
                Self::API_BASE,
                "simple/price?",
                self.query()
            ))
            .send()?
            .json::<T>()?;
        Ok(response)
    }
}

impl SimplePriceRequest {
    pub fn new(ids: String, vs_currencies: String) -> Self {
        Self {
            ids,
            vs_currencies,
            ..Default::default()
        }
    }

    pub fn query(&self) -> String {
        fomat!(
            "ids=" (self.ids)
            "&vs_currencies=" (self.vs_currencies)
            if (self.include_market_cap) {
                "&include_market_cap=true"
            }
            if (self.include_24hr_vol) {
                "&include_24hr_vol=true"
            }
            if (self.include_24hr_change) {
                "&include_24hr_change=true"
            }
            if (self.include_last_updated_at) {
                "&include_last_updated_at=true"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::GeckoClient;
    use anyhow::Context;

    #[test]
    fn test_get_simple_prices_correct() {
        let client = GeckoClient::new();

        let response =
            client.get_simple_prices(&["bitcoin", "ethereum", "cosmos"], &["usd", "ils", "eur"]);
        println!("The answer is: {:?}", response)
    }

    #[test]
    fn test_get_simple_prices_coin_not_found() {
        let client = GeckoClient::new();

        let response = client.get_simple_prices(&[""], &["usd"]);
        println!("The answer is: {:?}", response)
    }

    #[test]
    fn test_get_simple_prices_response_error() {
        let client = GeckoClient::new();

        let response = client
            .get_simple_prices(&["jkhg", "bitcoin"], &["usd"])
            .context(format!("Wrong inputs"));
        println!("The answer is: {:?}", response)
    }

    #[test]
    fn test_get_simple_prices_currency_error() {
        let client = GeckoClient::new();

        let response = client
            .get_simple_prices(&["bitcoin"], &["dfgfd"])
            .context(format!("Wrong inputs"));
        println!("The answer is: {:?}", response)
    }
}
