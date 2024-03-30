use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contest::data::bets::Bet;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserBetQueryResponse {
    pub bet: Bet,
}
