use thiserror::Error;

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
