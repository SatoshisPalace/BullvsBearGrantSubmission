use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct StatsResponse {
    pub contests: u32,
    pub bets: u64,
    pub users: u32,
    pub volume: Uint128,
}
