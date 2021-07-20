use std::collections::HashMap;

use crate::errors::SimpleResponseError;
use crate::generic::GeckoRequest;
use crate::ping::Ping;
use crate::simple::{SimplePriceRequest, SimplePriceResponse, SimplePrices};
use crate::utils::parse_str_args;

pub struct GeckoClient {}

impl GeckoClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_simple_prices(
        &self,
        coin_ids: &[&str],
        currencies: &[&str],
    ) -> Result<SimplePrices, SimpleResponseError> {
        let result: Result<SimplePrices, SimpleResponseError> =
            SimplePriceRequest::new(parse_str_args(coin_ids), parse_str_args(currencies))
                .get_json();
        let res = SimplePriceResponse {
            simple_response: result?,
        };
        res.validate_response(coin_ids, currencies)
    }
    pub fn ping(&self) -> Result<HashMap<String, String>, SimpleResponseError> {
        Ping::new().get_json()
    }
}
