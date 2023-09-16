use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use super::data::{contest_info::ContestInfo, contest_bet_summary::ContestBetSummary, bets::Bet};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestQueryResponse {
    pub contest_info: ContestInfo,
    pub contest_bet_summary: ContestBetSummary,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserBetQueryResponse {
    pub bet : Bet,
}