use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::bet_contest::BetContest;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvokeMsg {
    BetContest(BetContest),
}
