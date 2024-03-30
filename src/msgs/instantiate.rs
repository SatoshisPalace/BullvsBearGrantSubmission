use cosmwasm_std::{Addr, Binary, ContractInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub master_viewing_key_contract: ContractInfo,
    pub satoshis_palace: Addr,
    pub oracle_contract_info: ContractInfo,
    pub snip20: ContractInfo,
    pub entropy: Binary,
}
