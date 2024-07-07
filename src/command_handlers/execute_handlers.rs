use cosmwasm_std::{from_binary, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use sp_secret_toolkit::snip20::Snip20;

use crate::{
    contract::invoke,
    data::contest_info::ContestId,
    msgs::{
        execute::commands::{claim::Claim, claim_multiple::ClaimMultiple, receive::Receive},
        invoke::invoke_msg::InvokeMsg,
    },
    responses::execute::{
        execute_response::{ExecuteResponse, ResponseStatus::Success},
        response_types::claim::ClaimResponse,
    },
    services::{
        bet_service::user_claims_bet, contest_bet_summary_service::finalize_contest_outcome,
        contest_info_service::assert_contest_ready_to_be_claimed,
        state_service::assert_snip20_address, user_info_service::advance_index,
    },
};

pub fn handle_claim(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    command: Claim,
) -> StdResult<Response> {
    let Claim { contest_id } = command;
    let claimable_amount = process_claim(&mut deps, &env, &info, &contest_id)?;
    let snip20 = Snip20::singleton_load(deps.storage)?;

    Ok(Response::default()
        .add_message(snip20.create_send_msg(&info.sender.into_string(), &claimable_amount)?)
        .set_data(ExecuteResponse::Claim(ClaimResponse {
            status: Success,
            amount: claimable_amount,
        })))
}

pub fn handle_claim_multiple(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    command: ClaimMultiple,
) -> StdResult<Response> {
    let ClaimMultiple { mut contest_ids } = command;

    contest_ids.sort_by_key(|contest| *contest.time_of_close());

    let mut total_claimable_amount = Uint128::zero();

    for (index, contest_id) in contest_ids.iter().enumerate() {
        let claimable_amount = process_claim(&mut deps, &env, &info, contest_id)?;
        total_claimable_amount += claimable_amount;

        // Check if this is the last item
        if index == contest_ids.len() - 1 {
            advance_index(deps.storage, &info.sender, contest_id)?;
        }
    }

    let snip20 = Snip20::singleton_load(deps.storage)?;
    // reset_unchecked_contests_for_user(deps.storage, &info.sender);

    Ok(Response::default()
        .add_message(snip20.create_send_msg(&info.sender.into_string(), &total_claimable_amount)?)
        .set_data(ExecuteResponse::Claim(ClaimResponse {
            status: Success,
            amount: total_claimable_amount,
        })))
}

pub fn handle_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    command: Receive,
) -> StdResult<Response> {
    assert_snip20_address(deps.storage, &info.sender)?;

    let msg: InvokeMsg = from_binary(&command.msg)?;

    invoke(deps, env, info, msg, command.amount)
}

fn process_claim(
    deps: &mut DepsMut,
    env: &Env,
    info: &MessageInfo,
    contest_id: &ContestId,
) -> StdResult<Uint128> {
    let contest_info = assert_contest_ready_to_be_claimed(deps.storage, env, contest_id)?;

    let (contest_bet_summary, _was_finalized) = finalize_contest_outcome(deps, env, &contest_info)?;
    let claimable_amount = user_claims_bet(deps.storage, &info.sender, &contest_bet_summary)?;
    Ok(claimable_amount)
}
