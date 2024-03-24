use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::responses::query::{
    bet_response::UserBetQueryResponse,
    contest_response::{ContestInfoAndSummaryQueryResponse, ContestsQueryResponse},
};

// Enum to encapsulate each query response type
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    Contest(ContestInfoAndSummaryQueryResponse),
    Contests(ContestsQueryResponse),
    UserBet(UserBetQueryResponse),
}
