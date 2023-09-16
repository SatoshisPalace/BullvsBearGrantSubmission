use cosmwasm_std::{Addr, Uint128, Storage};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use crate::contest::{error::ContestError, constants::USER_CONTEST_CONFIG_KEY};




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserContest {
	address: Addr,
	contest_id : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Bet{
	pub amount: Uint128,
	pub outcome_id: u8
}

pub fn verify_bet(sender: &Option<Addr>, amount: Option<Uint128> )-> Result<(), ContestError>{
	if sender.is_none(){
		Err(ContestError::BetHasNoSender)
	}else if amount.is_none(){
		Err(ContestError::BetHasNoAmount)
	}else{
		Ok(())
	}
}

static BETS: Keymap<UserContest, Bet> = Keymap::new(USER_CONTEST_CONFIG_KEY);

////////
pub fn save_bet(storage: &mut dyn Storage, address: Addr, contest_id: u32, amount: Uint128, outcome_id: u8)-> Result<(), ContestError>{
	let user_contest = UserContest{
		address,
		contest_id,
	};
	let bet = Bet{
		amount,
		outcome_id,
	};
	BETS.insert(storage, &user_contest, &bet)?;
	Ok(())
}