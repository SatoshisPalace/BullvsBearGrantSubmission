use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::contest_info::ContestId;
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClaimMultiple {
    pub contest_ids: Vec<ContestId>,
}
