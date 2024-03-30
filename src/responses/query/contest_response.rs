use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contest::data::{contest_bet_summary::ContestBetSummary, contest_info::ContestInfo};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContestInfoAndSummaryQueryResponse {
    pub contest_info: ContestInfo,
    pub contest_bet_summary: ContestBetSummary,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContestsQueryResponse {
    pub contests: Vec<ContestInfoAndSummaryQueryResponse>,
}
