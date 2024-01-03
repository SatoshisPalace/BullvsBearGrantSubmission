use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Storage, ContractInfo};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"oracle_state";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OracleState {
    pub oracle_contract_info: ContractInfo,
}

fn config(storage: &mut dyn Storage) -> Singleton<OracleState> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<OracleState> {
    singleton_read(storage, CONFIG_KEY)
}


pub fn initialize_orace_state(
    storage: &mut dyn Storage,
    oracle_contract_info: ContractInfo,
){
    let orace_state = OracleState {
        oracle_contract_info,
    };

    config(storage).save(&orace_state).unwrap();
}
