#[macro_use]
extern crate fomat_macros;

mod endpoints;

use url::{Url, ParseError};
use reqwest;
use std::collections::HashMap;
use serde_json::Value;
use anyhow::Result as AnyhowResult;

const API_BASE: &str = "https://api.coingecko.com/api/v3/";

fn parse_url(endpoint: &str) -> Result<Url, ParseError> {
    let result = Url::parse(crate::API_BASE)?
        .join(endpoint)?;
    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::{Ping, GeckoRequest, SimplePrice, SimplePriceResponse, ResponseError};
    use std::fmt::Debug;
    use std::hash::Hash;
    use serde::de::DeserializeOwned;

    #[test]
    fn test_equal_urls() {
        assert_eq!(parse_url("simple/price").unwrap().as_str(), "https://api.coingecko.com/api/v3/simple/price");
    }

    #[tokio::test]
    async fn test_ping() {
        let client = reqwest::Client::new();
        let res = client.get(parse_url("ping").unwrap().as_str())
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

    #[test]
    fn test_simpleprice_with_trait() {
        let result: SimplePriceResponse = SimplePrice::new("bitcoin,ethereum".to_string(), "usd,ils".to_string()).get_json().unwrap();
        println!("body = {:?}", result);
        //if let Some(ans) = result.0.get("bitcoin").unwrap().get("usd") {
        //println!("The price of Bitcoin is: {:?} USD", result.0.get("bitcoin").unwrap().get("usd").unwrap());
        if let Err(e) = result.get("bitcoinuioij", "usd") {
            eprintln!("{}", e)
        };
        if let Err(e) = result.get("bitcoin", "xxx") {
            assert_eq!(e, ResponseError::GetRequestCurrency("xxx".to_string()))
        };
        println!("The price of Bitcoin is: {:?} USD", result.get("bitcoin", "usd").unwrap());
    }
}