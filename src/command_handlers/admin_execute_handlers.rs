use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Uint128};
use sp_secret_toolkit::snip20::Snip20;

use crate::responses::execute::execute_response::ResponseStatus::Success;
use crate::{
    data::state::State,
    msgs::execute::commands::set_minimum_bet::SetMinimumBet,
    responses::execute::{execute_response::ExecuteResponse, response_types::claim::ClaimResponse},
    services::state_service::get_claimable_fees,
};

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

pub fn handle_claim_fees(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut state = State::singleton_load(deps.storage)?;
    let snip20 = Snip20::singleton_load(deps.storage)?;

    state.assert_owner(&info.sender)?;

    let claimable_amount = get_claimable_fees(deps.storage).unwrap();

    state.set_claimable_fees(Uint128::zero());
    state.singleton_save(deps.storage)?;

    Ok(Response::default()
        .add_message(snip20.create_send_msg(&info.sender.into_string(), &claimable_amount)?)
        .set_data(ExecuteResponse::Claim(ClaimResponse {
            status: Success,
            amount: claimable_amount,
        })))
}
