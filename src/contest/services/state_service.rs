use cosmwasm_std::Uint128;

use crate::{contest::error::state_error::StateError, state::State};

pub fn get_minimum_bet(storage: &dyn cosmwasm_std::Storage) -> Result<Uint128, StateError> {
    let state = State::singleton_load(storage)?;
    Ok(state.get_minimum_bet().clone())
}
