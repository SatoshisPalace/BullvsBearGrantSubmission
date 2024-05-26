use core::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

use crate::services::integrations::price_feed_service::pricefeed::NULL_AND_VOID_CONTEST_RESULT;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct ContestInfo {
    ticker: String,
    options: Vec<ContestOutcome>,
    time_of_close: u64,
    time_of_resolve: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestId {
    ticker: String,
    time_of_close: u64
}
impl ContestId {
    pub fn new(ticker: String, time_of_close: u64) -> Self {
        ContestId { ticker, time_of_close }
    }
}

impl fmt::Display for ContestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ticker: {}, Time of Close: {}", self.ticker, self.time_of_close)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestOutcome {
    id: u8,
    name: String,
}
impl ContestOutcome {
    pub fn new(id: u8, name: String) -> Self {
        ContestOutcome { id, name }
    }
    pub fn nullified_result() -> Self {
        ContestOutcome {
            id: NULL_AND_VOID_CONTEST_RESULT,
            name: "Nullified Result".to_string(),
        }
    }
    pub fn get_id(&self) -> &u8 {
        &self.id
    }
}

impl ContestInfo {
    pub fn new(
        ticker: String,
        time_of_close: u64,
        time_of_resolve: u64,
        options: Vec<ContestOutcome>,
    ) -> ContestInfo {
        Self {
            ticker,
            time_of_close,
            time_of_resolve,
            options,
        }
    }
    pub fn get_id(&self) -> ContestId {
        return self.id()
    }
    pub fn get_ticker(&self) -> String {
        return self.ticker.clone();
    }
    pub fn get_time_of_close(&self) -> u64 {
        return self.time_of_close;
    }
    pub fn get_time_of_resolve(&self) -> u64 {
        return self.time_of_resolve;
    }
    pub fn get_options(&self) -> &Vec<ContestOutcome> {
        return &self.options;
    }
}

impl Identifiable for ContestInfo {
    type ID = ContestId;

    fn id(&self) -> Self::ID {
        ContestId {
            ticker: self.ticker.clone(),
            time_of_close: self.time_of_close
        }
    } // Or another type that implements Serialize + DeserializeOwned
}
