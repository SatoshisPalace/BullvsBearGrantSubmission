use cosmwasm_std::{Addr, Storage, Uint128};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use crate::contest::{constants::USER_CONTEST_CONFIG_KEY, error::ContestError};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserContest {
    pub address: Addr,
    pub contest_id: u32,
}

impl UserContest {
    pub fn get_address_as_str(&self) -> &str {
        return self.address.as_str();
    }
    pub fn get_contest_id(&self) -> u32 {
        return self.contest_id;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Bet {
    pub amount: Uint128,
    pub outcome_id: u8,
    pub has_been_paid: bool
}

impl Bet {
    pub fn assert_not_paid(&self) -> Result<(), ContestError> {
        if self.has_been_paid {
            Err(ContestError::BetAlreadyPaid)
        } else {
            Ok(())
        }
    }
}

pub fn verify_bet(sender: &Option<Addr>, amount: Option<Uint128>) -> Result<(), ContestError> {
    if sender.is_none() {
        Err(ContestError::BetHasNoSender)
    } else if amount.is_none() {
        Err(ContestError::BetHasNoAmount)
    } else {
        Ok(())
    }
}

static BETS: Keymap<UserContest, Bet> = Keymap::new(USER_CONTEST_CONFIG_KEY);

////////
pub fn save_bet(
    storage: &mut dyn Storage,
    address: Addr,
    contest_id: u32,
    amount: Uint128,
    outcome_id: u8,
    has_been_paid: bool
) -> Result<(), ContestError> {
    let user_contest = UserContest {
        address,
        contest_id,
    };
    let bet = Bet { amount, outcome_id, has_been_paid};
    BETS.insert(storage, &user_contest, &bet)?;
    Ok(())
}

pub fn get_bet(storage: &dyn Storage, user_contest: &UserContest) -> Option<Bet> {
    return BETS.get(storage, user_contest);
}
