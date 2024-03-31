use cosmwasm_std::{from_binary, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use sp_secret_toolkit::snip20::Snip20;

use crate::{
    contract::invoke,
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
        state_service::assert_snip20_address,
    },
};

pub fn handle_claim(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    command: Claim,
) -> StdResult<Response> {
    let Claim { contest_id } = command;

    let contest_info = assert_contest_ready_to_be_claimed(deps.storage, &env, &contest_id)?;
    let contest_bet_summary = finalize_contest_outcome(&mut deps, &env, &contest_info)?;
    let claimable_amount = user_claims_bet(deps.storage, &info.sender, &contest_bet_summary)?;

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
    let ClaimMultiple { contest_ids } = command;

    // Initialize total claimable amount
    let mut total_claimable_amount = Uint128::zero();

    for contest_id in contest_ids.iter() {
        // For each contest ID, perform the operations to calculate the claimable amount
        let contest_info = assert_contest_ready_to_be_claimed(deps.storage, &env, contest_id)?;
        let contest_bet_summary = finalize_contest_outcome(&mut deps, &env, &contest_info)?;
        let claimable_amount = user_claims_bet(deps.storage, &info.sender, &contest_bet_summary)?;

        // Sum up the claimable amounts
        total_claimable_amount += claimable_amount;
    }

    // Assuming Snip20::singleton_load(deps.storage)? loads an instance to interact with SNIP-20 token contract
    let snip20 = Snip20::singleton_load(deps.storage)?;

    // Send the total claimable amount in one Snip20 message
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
