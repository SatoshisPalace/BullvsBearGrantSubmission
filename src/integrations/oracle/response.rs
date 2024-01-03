use schemars::JsonSchema;
use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestResult {
    pub side_a: u8,
    pub side_b: u8,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct GetContestResultResponse {
    pub result: u8,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct GetContestStatusResponse {
    pub contest_status: ContestResult,
}
