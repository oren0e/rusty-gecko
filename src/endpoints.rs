use anyhow::Result as AnyhowResult;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub type Price = f32;
pub type Coin = String; // type for crypto coin
pub type Currency = String; // type for currency for displaying coins in
pub type SimplePrice = HashMap<Currency, Price>; // <Currency, Price>
pub type SimplePrices = HashMap<Coin, SimplePrice>; // <Coin, TypeSimplePrice>

pub trait GeckoRequest {
    const API_BASE: &'static str = "https://api.coingecko.com/api/v3/";
    fn get_json<T: DeserializeOwned>(&self) -> Result<T, SimpleResponseError>;
}

#[derive(Debug, Deserialize, Default)]
pub struct Ping {
    pub gecko_says: String,
}

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
// check for error - returns SimplePrices (with no errors)
//

#[derive(Error, Debug)]
pub enum SimpleResponseError {
    #[error("No data in input")]
    EmptyInputError,

    #[error("No such coin found! (`{0}`)")]
    UnknownCoinError(String),

    #[error("No such currency found! (`{0}`)")]
    UnknownCurrencyError(String),

    #[error("HTTP Error")]
    HttpError(#[from] reqwest::Error),

    #[error("IO Error")]
    IOError(#[from] std::io::Error),

    #[error("Deserialization Error")]
    DeserializationError(#[from] serde_json::Error),
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
        }
        return Ok(self.simple_response.clone());
    }

    //    pub fn get<S: AsRef<str>>(&self, coin: S, in_currency: S) -> Result<&f32, ResponseError> {
    //        let coin = self
    //            .0
    //            .get(coin.as_ref())
    //            .ok_or(ResponseError::GetRequestCoin(coin.as_ref().to_string()))?;
    //        let currency = coin
    //            .get(in_currency.as_ref())
    //            .ok_or(ResponseError::GetRequestCurrency(
    //                in_currency.as_ref().to_string(),
    //            ))?;
    //        Ok(currency)
    //    }
}

impl GeckoRequest for Ping {
    fn get_json<T: DeserializeOwned>(&self) -> Result<T, SimpleResponseError> {
        let response: T = Client::new()
            .get(format!("{}{}", Self::API_BASE, "ping"))
            .send()?
            .json::<T>()?;
        Ok(response)
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

impl Ping {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
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
