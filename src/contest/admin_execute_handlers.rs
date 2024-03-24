use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Uint128};

use crate::state::State;

pub fn handle_set_minimum_bet(
    deps: DepsMut,
    info: MessageInfo,
    min_bet_amount: Uint128,
) -> StdResult<Response> {
    let mut state = State::singleton_load(deps.storage)?;
    state.assert_owner(&info.sender)?;

    state.set_minimum_bet(min_bet_amount);
    state.singleton_save(deps.storage)?;
    Ok(Response::default())
}
