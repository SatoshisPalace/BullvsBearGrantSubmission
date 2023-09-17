use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

use super::snip_20_contract::Snip20Contract;

pub static CONFIG_KEY: &[u8] = b"snip_20_config";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Snip20State {
    pub known_snip_20: Vec<Snip20Contract>,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<Snip20State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Snip20State> {
    singleton_read(storage, CONFIG_KEY)
}
