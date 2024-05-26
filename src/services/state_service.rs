use cosmwasm_std::{Addr, Api, Storage, Uint128};
use sp_secret_toolkit::{
    contract::contract::Contract, cryptography::signing::is_valid_signature, snip20::Snip20,
};

use crate::{
    data::{
        contest_info::ContestInfo,
        state::{FeePercent, State},
    },
    error::state_error::StateError,
};

pub fn get_fee_percent(storage: &dyn cosmwasm_std::Storage) -> Result<FeePercent, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.fee_percent().clone())
}

pub fn get_minimum_bet(storage: &dyn cosmwasm_std::Storage) -> Result<Uint128, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.minimum_bet().clone())
}

pub fn get_claimable_fees(storage: &dyn cosmwasm_std::Storage) -> Result<Uint128, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.claimable_fees().clone())
}

pub fn add_claimable_fee_for_pool(storage: &mut dyn cosmwasm_std::Storage, total_pool: &Uint128) {
    let mut state = State::singleton_load(storage).unwrap();
    let current_fees = state.claimable_fees().to_owned();
    let fee = state.fee_percent().to_owned();

    let fee_amount = total_pool.u128()
        - (total_pool.u128() * (fee.denominator() - fee.numerator()) / fee.denominator());

    let new_collected_fees = current_fees + Uint128::from(fee_amount);

    state.set_claimable_fees(new_collected_fees);
    let _ = state.singleton_save(storage);
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

pub fn assert_contest_info_signature_is_valid(
    storage: &dyn Storage,
    api: &dyn Api,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), StateError> {
    let state = State::singleton_load(storage)?;

    let contest_info_json: String = contest_info.to_json();
    is_valid_signature(
        api,
        state.satoshis_palace().as_str(),
        &contest_info_json,
        &contest_info_signature_hex,
    )?;
    Ok(())
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
