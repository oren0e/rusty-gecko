#[macro_use]
extern crate fomat_macros;

mod endpoints;
mod utils;

use anyhow::Result as AnyhowResult;
use endpoints::{
    GeckoRequest, SimplePrice, SimplePriceRequest, SimplePriceResponse, SimplePrices,
    SimpleResponseError,
};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use url::{ParseError, Url};
use utils::parse_str_args;

const API_BASE: &str = "https://api.coingecko.com/api/v3/";

fn parse_url(endpoint: &str) -> Result<Url, ParseError> {
    let result = Url::parse(crate::API_BASE)?.join(endpoint)?;
    Ok(result)
}

pub struct GeckoClient {}

impl GeckoClient {
    pub fn new() -> Self {
        Self {}
    }

    // this returns the hashmap as it is because it asks for many coins and currencies
    // It might be worth adding another method for a single coin and currency to strip one
    // layer and return SimplePrice, then in that method I can do a hashmap lookup for the desired
    // key
    pub fn get_simple_prices(
        &self,
        coin_ids: &[&str],
        currencies: &[&str],
    ) -> AnyhowResult<SimplePrices> {
        let result: AnyhowResult<SimplePrices, SimpleResponseError> =
            SimplePriceRequest::new(parse_str_args(coin_ids), parse_str_args(currencies))
                .get_json()?;
        let res = SimplePriceResponse {
            simple_response: result?,
        };
        res.validate_response(coin_ids, currencies)
        // Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::{
        GeckoRequest, Ping, SimplePriceRequest, SimplePriceResponse, SimpleResponseError,
    };
    use anyhow::Context;
    use serde::de::DeserializeOwned;
    use std::fmt::Debug;
    use std::hash::Hash;

    #[test]
    fn test_equal_urls() {
        assert_eq!(
            parse_url("simple/price").unwrap().as_str(),
            "https://api.coingecko.com/api/v3/simple/price"
        );
    }

    #[tokio::test]
    async fn test_ping() {
        let client = reqwest::Client::new();
        let res = client
            .get(parse_url("ping").unwrap().as_str())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("body = {:?}", res)
    }

    // #[tokio::test]
    // async fn test_ping_struct() {
    //     let client = reqwest::Client::new();
    //     let ping_request = utils::get_json(client, parse_url("ping").unwrap()).await.unwrap();
    //     println!("{:#?}", ping_request)
    // }

    // #[tokio::test]
    // async fn test_ping1() {
    //     let client = reqwest::Client::new();
    //     let res = client.get(parse_url("ping").unwrap().as_str())
    //         .send()
    //         .await
    //         .unwrap();
    //     let ping: endpoints::Ping = res.json().await.unwrap();
    //         // .json::<endpoints::Ping>()
    //         // .await
    //         // .unwrap();
    //     println!("body = {:?}", ping)
    // }
    #[test]
    fn test_ping_with_trait() {
        let ping_result: Ping = Ping::new().get_json().unwrap();
        println!("body = {:?}", ping_result)
    }

    //    #[test]
    //    fn test_simpleprice_with_trait() {
    //        let result: SimplePriceResponse =
    //            SimplePriceRequest::new("bitcoin,ethereum".to_string(), "usd,ils".to_string())
    //                .get_json()
    //                .unwrap();
    //        println!("body = {:?}", result);
    //        //if let Some(ans) = result.0.get("bitcoin").unwrap().get("usd") {
    //        //println!("The price of Bitcoin is: {:?} USD", result.0.get("bitcoin").unwrap().get("usd").unwrap());
    //        if let Err(e) = result.get("bitcoinuioij", "usd") {
    //            eprintln!("{}", e)
    //        };
    //        if let Err(e) = result.get("bitcoin", "xxx") {
    //            assert_eq!(e, ResponseError::GetRequestCurrency("xxx".to_string()))
    //        };
    //        println!(
    //            "The price of Bitcoin is: {:?} USD",
    //            result.get("bitcoin", "usd").unwrap()
    //        );
    //    }

    #[test]
    fn test_gecko_client_simple_price() {
        let client = GeckoClient::new();
        let response = match client.get_simple_prices(&["bitcoin"], &["usd"]) {
            Ok(ans) => println!("The prices of of the supplied coins are: {:?}", ans),
            Err(e) => panic!("Error: {:?}", e),
        };
    }

    #[test]
    fn test_gecko_client_simple_prices() {
        let client = GeckoClient::new();

        if let Ok(response) =
            client.get_simple_prices(&["bitcoin", "ethereum", "cosmos"], &["usd", "ils", "eur"])
        {
            println!("The answer is: {:?}", response)
        }
    }

    #[test]
    fn test_gecko_client_simple_prices_coin_not_found() {
        let client = GeckoClient::new();

        if let Ok(response) = client.get_simple_prices(&[""], &["usd"]) {
            println!("The answer is: {:?}", response)
        }
    }

    #[test]
    fn test_get_simple_prices_response_error() {
        let client = GeckoClient::new();

        let response = client
            .get_simple_prices(&["bitcoin"], &["usd"])?
            .context(format!("Wrong inputs"));
        println!("The answer is: {:?}", response)
    }
}
