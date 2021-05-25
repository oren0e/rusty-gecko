use anyhow::Result as AnyhowResult;
use reqwest::blocking::Client;
use serde::__private::de::IdentifierDeserializer;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub type TypeSimplePrice = HashMap<String, f32>; // <Currency, Price>
pub type TypeSimplePrices = HashMap<String, TypeSimplePrice>; // <Coin, TypeSimplePrice>

pub trait GeckoRequest {
    const API_BASE: &'static str = "https://api.coingecko.com/api/v3/";
    fn get_json<T: DeserializeOwned>(&self) -> AnyhowResult<T>;
}

#[derive(Debug, Deserialize, Default)]
pub struct Ping {
    pub gecko_says: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct SimplePrice {
    pub ids: String,
    pub vs_currencies: String,
    pub include_market_cap: bool,
    pub include_24hr_vol: bool,
    pub include_24hr_change: bool,
    pub include_last_updated_at: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimplePriceResponse(pub TypeSimplePrices);

#[derive(Serialize, Deserialize, Error, Debug, PartialOrd, PartialEq)]
pub enum ResponseError {
    #[error("GET Error: No such coin found! ({0})")]
    GetRequestCoin(String),
    #[error("GET Error: No currency found! ({0})")]
    GetRequestCurrency(String),
}

impl SimplePriceResponse {
    pub fn get<S: AsRef<str>>(&self, coin: S, in_currency: S) -> Result<&f32, ResponseError> {
        let coin = self
            .0
            .get(coin.as_ref())
            .ok_or(ResponseError::GetRequestCoin(coin.as_ref().to_string()))?;
        let currency = coin
            .get(in_currency.as_ref())
            .ok_or(ResponseError::GetRequestCurrency(
                in_currency.as_ref().to_string(),
            ))?;
        Ok(currency)
    }
}

impl GeckoRequest for Ping {
    fn get_json<T: DeserializeOwned>(&self) -> AnyhowResult<T> {
        let response: T = Client::new()
            .get(format!("{}{}", Self::API_BASE, "ping"))
            .send()?
            .json::<T>()?;
        Ok(response)
    }
}

impl GeckoRequest for SimplePrice {
    fn get_json<T: DeserializeOwned>(&self) -> AnyhowResult<T> {
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

impl SimplePrice {
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
