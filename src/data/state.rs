use getset::{Getters, Setters};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use sp_secret_toolkit::macros::singleton::SingletonStorage;

use crate::error::state_error::StateError;

#[derive(Getters, Setters, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[getset(get = "pub", set = "pub")]
pub struct FeePercent {
    numerator: u128,
    denominator: u128,
}

impl FeePercent {
    pub fn new(numerator: u128, denominator: u128) -> Self {
        FeePercent {
            numerator,
            denominator,
        }
    }
}

#[derive(
    Getters,
    Setters,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    JsonSchema,
    SingletonStorage,
)]
#[getset(get = "pub", set = "pub")]
pub struct State {
    satoshis_palace: Addr,
    owner: Addr,
    minimum_bet: Uint128,
    fee_percent: FeePercent,
    claimable_fees: Uint128,
}

impl State {
    // Constructor
    pub fn new(
        satoshis_palace: Addr,
        owner: Addr,
        amount: Uint128,
        fee_percent: FeePercent,
    ) -> Self {
        State {
            satoshis_palace,
            owner,
            minimum_bet: amount,
            fee_percent,
            claimable_fees: Uint128::zero(),
        }
    }

    // Assertions
    pub fn assert_owner(&self, caller: &Addr) -> Result<(), StateError> {
        if self.owner().to_string() == caller.to_string() {
            Ok(())
        } else {
            Err(StateError::Unauthorized {
                expected: self.owner().to_string(),
                actual: caller.to_string(),
            })
        }
    }
}
