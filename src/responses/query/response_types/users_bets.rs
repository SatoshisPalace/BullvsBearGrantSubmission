use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::{bets::Bet, contest_bet_summary::ContestBetSummary, contest_info::ContestInfo};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserContestBetInfo {
    pub contest_info: ContestInfo,
    pub contest_bet_summary: ContestBetSummary,
    pub user_bet: Bet,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsersBetsResponse {
    pub contests_bets: Vec<UserContestBetInfo>,
}
