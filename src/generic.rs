use serde::de::DeserializeOwned;

use crate::errors::SimpleResponseError;

pub trait GeckoRequest {
    const API_BASE: &'static str = "https://api.coingecko.com/api/v3/";
    fn get_json<T: DeserializeOwned>(&self) -> Result<T, SimpleResponseError>;
}
