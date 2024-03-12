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
pub struct ContestsQueryResponse {
    contest_query_responses: Vec<ContestQueryResponse>,
}

impl ContestsQueryResponse {
    pub fn new(contest_query_responses: Vec<ContestQueryResponse>) -> Self {
        Self {
            contest_query_responses,
        }
    }
    // Method to get the length of contest_query_responses
    pub fn len(&self) -> usize {
        self.contest_query_responses.len()
    }

    // Method to index into contest_query_responses
    pub fn get(&self, index: usize) -> Option<&ContestQueryResponse> {
        self.contest_query_responses.get(index)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserBetQueryResponse {
    pub bet: Bet,
}

// Enum to encapsulate each query response type
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    Contest(ContestQueryResponse),
    Contests(ContestsQueryResponse),
    UserBet(UserBetQueryResponse),
}
