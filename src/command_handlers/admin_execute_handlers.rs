use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::{data::state::State, msgs::execute::commands::set_minimum_bet::SetMinimumBet};

pub fn handle_set_minimum_bet(
    deps: DepsMut,
    info: MessageInfo,
    command: SetMinimumBet,
) -> StdResult<Response> {
    let mut state = State::singleton_load(deps.storage)?;
    state.assert_owner(&info.sender)?;

    state.set_minimum_bet(command.amount);
    state.singleton_save(deps.storage)?;
    Ok(Response::default())
}
