use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::bets::Bet;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserBetResponse {
    pub bet: Bet,
}
