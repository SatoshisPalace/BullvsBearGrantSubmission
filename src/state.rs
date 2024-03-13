use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, CanonicalAddr};
use sp_secret_toolkit::macros::singleton::SingletonStorage;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, SingletonStorage)]
pub struct State {
    pub satoshis_palace: Addr,
    pub owner: CanonicalAddr,
}
