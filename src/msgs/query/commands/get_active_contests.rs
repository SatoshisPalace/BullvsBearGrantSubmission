use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetActiveContests {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_order: Option<ContestQuerySortOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContestQuerySortOrder {
    Volume,
}
