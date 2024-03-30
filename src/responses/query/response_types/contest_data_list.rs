use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::contest_data::ContestDataResponse;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContestDataListResponse {
    pub contests: Vec<ContestDataResponse>,
}
