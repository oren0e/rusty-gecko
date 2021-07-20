use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::errors::SimpleResponseError;
use crate::generic::GeckoRequest;

#[derive(Debug, Deserialize, Default)]
pub struct Ping {
    pub gecko_says: String,
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

impl Ping {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::GeckoClient;

    #[test]
    fn test_ping() {
        let client = GeckoClient::new();

        let response = client.ping();
        println!("Ping worked, the answer is: {:?}", response)
    }
}
