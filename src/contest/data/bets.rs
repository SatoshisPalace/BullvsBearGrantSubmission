use std::fmt;

use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

use crate::contest::error::ContestError;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct Bet {
    user: Addr,
    contest_id: u32,
    amount: Uint128,
    outcome_id: u8,
    has_been_paid: bool,
}

impl Bet {
    pub fn new(user: Addr, contest_id: u32, amount: Uint128, outcome_id: u8) -> Self {
        Bet {
            user,
            contest_id,
            amount,
            outcome_id,
            has_been_paid: false, // Bets are not paid when they're created
        }
    }

    // Getters
    pub fn get_user(&self) -> &Addr {
        &self.user
    }

    pub fn get_contest_id(&self) -> &u32 {
        &self.contest_id
    }

    pub fn get_amount(&self) -> &Uint128 {
        &self.amount
    }

    pub fn get_outcome_id(&self) -> &u8 {
        &self.outcome_id
    }

    pub fn has_been_paid(&self) -> &bool {
        &self.has_been_paid
    }

    // Setter for has_been_paid
    pub fn mark_paid(&mut self) {
        self.has_been_paid = true;
    }

    // Adder for amount
    pub fn add_amount(&mut self, additional_amount: Uint128) {
        self.amount += additional_amount;
    }

    pub fn assert_not_paid(&self) -> Result<(), ContestError> {
        if self.has_been_paid {
            Err(ContestError::BetAlreadyPaid)
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserContest {
    address: Addr,
    contest_id: u32,
}

impl UserContest {
    pub fn new(address: Addr, contest_id: u32) -> Self {
        UserContest {
            address,
            contest_id,
        }
    }

    pub fn get_address_as_str(&self) -> &str {
        return self.address.as_str();
    }
    pub fn get_contest_id(&self) -> &u32 {
        return &self.contest_id;
    }

    pub fn get_address(&self) -> &Addr {
        &self.address
    }
}

impl Identifiable for Bet {
    type ID = UserContest; // Or another type that implements Serialize + DeserializeOwned

    fn id(&self) -> Self::ID {
        return UserContest::new(self.get_user().to_owned(), self.get_contest_id().to_owned());
    }
}

impl fmt::Display for UserContest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address: {}, Contest ID: {}",
            self.address, self.contest_id
        )
    }
}
