use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::{claim::Claim, receive::Receive, set_minimum_bet::SetMinimumBet};
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Claim(Claim),
    SetMinimumBet(SetMinimumBet),
    Receive(Receive),
}
