use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use sp_secret_toolkit::macros::singleton::SingletonStorage;

use crate::error::state_error::StateError;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, SingletonStorage)]
pub struct State {
    satoshis_palace: Addr,
    owner: Addr,
    minimum_bet: Uint128,
}

impl State {
    // Constructor
    pub fn new(satoshis_palace: Addr, owner: Addr, amount: Uint128) -> Self {
        State {
            satoshis_palace,
            owner,
            minimum_bet: amount,
        }
    }

    // Getters
    pub fn get_satoshis_palace_signing_address(&self) -> &Addr {
        &self.satoshis_palace
    }

    pub fn get_owner(&self) -> &Addr {
        &self.owner
    }

    pub fn get_minimum_bet(&self) -> &Uint128 {
        &self.minimum_bet
    }

    // Setters
    pub fn set_satoshis_palace(&mut self, satoshis_palace: Addr) {
        self.satoshis_palace = satoshis_palace;
    }

    pub fn set_owner(&mut self, owner: Addr) {
        self.owner = owner;
    }

    pub fn set_minimum_bet(&mut self, amount: Uint128) {
        self.minimum_bet = amount;
    }

    // Assertions
    pub fn assert_owner(&self, caller: &Addr) -> Result<(), StateError> {
        if self.get_owner().to_string() == caller.to_string() {
            Ok(())
        } else {
            Err(StateError::Unauthorized {
                expected: self.get_owner().to_string(),
                actual: caller.to_string(),
            })
        }
    }
}
