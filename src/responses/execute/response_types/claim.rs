use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::responses::execute::execute_response::ResponseStatus;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClaimResponse {
    pub status: ResponseStatus,
    pub amount: Uint128,
}
