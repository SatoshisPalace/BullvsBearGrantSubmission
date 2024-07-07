use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TotalUsersNumberOfBetsResponse {
    pub total_users_number_of_bets: u32,
}
