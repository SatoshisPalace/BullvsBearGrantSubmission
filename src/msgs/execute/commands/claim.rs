use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::identifiable::Identifiable;

use crate::data::contest_info::ContestInfo;
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Claim {
    pub contest_id: <ContestInfo as Identifiable>::ID,
}
