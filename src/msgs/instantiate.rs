use cosmwasm_std::{Binary, ContractInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub master_viewing_key_contract: ContractInfo,
    pub price_feed_info: ContractInfo,
    pub snip20: ContractInfo,
    pub interval: u64,
    pub entropy: Binary,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
}
