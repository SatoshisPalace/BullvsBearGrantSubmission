use cosmwasm_std::{to_binary, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::response_types::{
    bet::BetResonse, claim::ClaimResponse,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteResponse {
    Claim(ClaimResponse),
    Bet(BetResonse),
}

impl From<ExecuteResponse> for Binary {
    fn from(answer: ExecuteResponse) -> Self {
        to_binary(&answer).unwrap() // Converts `ExecuteAnswer` to `Binary`
    }
}
