use cosmwasm_std::{Binary, to_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteAnswer {
    CreateContestAnswer {
        status: ResponseStatus,
    },
    BetContestAnswer {
        status: ResponseStatus,
    },
    ClaimContestAnswer {
        status: ResponseStatus,
    },
}

impl From<ExecuteAnswer> for Binary {
    fn from(answer: ExecuteAnswer) -> Self {
        to_binary(&answer).unwrap() // Converts `ExecuteAnswer` to `Binary`
    }
}