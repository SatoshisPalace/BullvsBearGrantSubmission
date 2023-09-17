use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::data::{
    bets::Bet,
    contest_bet_summary::ContestBetSummary,
    contest_info::{ContestInfo, ContestOutcome},
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestQueryResponse {
    pub contest_info: ContestInfo,
    pub contest_bet_summary: ContestBetSummary,
    pub contest_winner: ContestOutcome,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserBetQueryResponse {
    pub bet: Bet,
}
