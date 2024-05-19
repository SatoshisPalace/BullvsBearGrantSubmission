use cosmwasm_std::{Addr, Storage, Uint128};
use sp_secret_toolkit::{
    contract::contract::Contract, snip20::Snip20,
};

use crate::{
    data::state::State,
    error::state_error::StateError,
};

pub fn get_minimum_bet(storage: &dyn cosmwasm_std::Storage) -> Result<Uint128, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.get_minimum_bet().clone())
}

pub fn get_interval(storage: &dyn cosmwasm_std::Storage) -> Result<u64, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.get_interval().clone())
}

pub fn assert_amount_is_greater_than_minimum_bet(
    storage: &dyn cosmwasm_std::Storage,
    amount: &Uint128,
) -> Result<Uint128, StateError> {
    let minimum_bet = get_minimum_bet(storage)?;
    if amount >= &minimum_bet {
        Ok(minimum_bet)
    } else {
        Err(StateError::BetBelowMinimum {
            attempted: amount.to_owned(),
            minimum: minimum_bet,
        })
    }
}

pub fn get_snip20(storage: &dyn Storage) -> Result<Snip20, StateError> {
    let snip20 = Snip20::singleton_load(storage)?;
    Ok(snip20)
}

pub fn assert_snip20_address(storage: &dyn Storage, address: &Addr) -> Result<Snip20, StateError> {
    let snip20 = get_snip20(storage)?;
    Contract::assert_address(&snip20, address.clone())?;
    Ok(snip20)
}
