use std::fmt;

use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use secret_toolkit::storage::Item;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

use super::contest_info::{ContestId, ContestInfo};

pub static TOTAL_VOLUME: Item<Uint128> = Item::new(b"TOTAL_VOLUME");
pub static TOTAL_BETS: Item<u64> = Item::new(b"TOTAL_BETS");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct Bet {
    user: Addr,
    contest_id: <ContestInfo as Identifiable>::ID,
    amount: Uint128,
    outcome_id: u8,
    has_been_paid: bool,
}

impl Bet {
    pub fn new(
        user: Addr,
        contest_id: <ContestInfo as Identifiable>::ID,
        amount: Uint128,
        outcome_id: u8,
    ) -> Self {
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

    pub fn get_contest_id(&self) -> &ContestId {
        &self.contest_id
    }

    pub fn get_amount(&self) -> &Uint128 {
        &self.amount
    }

    pub fn get_outcome_id(&self) -> &u8 {
        &self.outcome_id
    }

    pub fn has_been_paid(&self) -> bool {
        self.has_been_paid
    }

    // Setter for has_been_paid
    pub fn mark_paid(&mut self) {
        self.has_been_paid = true;
    }

    // Adder for amount
    pub fn add_amount(&mut self, additional_amount: Uint128) {
        self.amount += additional_amount;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserContest {
    user: Addr,
    contest_id: <ContestInfo as Identifiable>::ID,
}

impl UserContest {
    pub fn new(user: Addr, contest_id: <ContestInfo as Identifiable>::ID) -> Self {
        UserContest { user, contest_id }
    }

    pub fn get_address_as_str(&self) -> &str {
        return self.user.as_str();
    }
    pub fn get_contest_id(&self) -> &ContestId {
        return &self.contest_id;
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
        write!(f, "Address: {}, Contest ID: {}", self.user, self.contest_id)
    }
}
