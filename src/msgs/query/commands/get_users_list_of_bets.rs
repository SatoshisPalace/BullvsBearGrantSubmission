use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetUsersListOfBets {
    pub user: Addr,
    pub viewing_key: String,
    pub contest_ids: Vec<u32>,
}
