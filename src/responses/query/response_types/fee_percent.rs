use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::state::FeePercent;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct FeePercentResponse {
    pub fee_percent: FeePercent,
}
