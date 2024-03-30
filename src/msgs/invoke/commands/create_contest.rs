use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data::contest_info::ContestInfo;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateContest {
    pub contest_info: ContestInfo,
    pub contest_info_signature_hex: String,
    pub outcome_id: u8,
    pub user: Addr,
}
