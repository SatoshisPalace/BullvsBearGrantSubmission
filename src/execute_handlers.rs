use cosmwasm_std::{Addr, DepsMut, Env, Response, StdResult, Uint128};
use sp_secret_toolkit::snip20::Snip20;

use crate::answer::{ExecuteAnswer, ResponseStatus};

use super::{
    data::contest_info::ContestInfo,
    services::{
        bet_service::{place_or_update_bet, user_claims_bet},
        contest_bet_summary_service::{
            add_bet_to_contest_summary, create_new_contest_bet_summary, finalize_contest_outcome,
        },
        contest_info_service::{assert_contest_ready_to_be_claimed, create_new_contest},
        user_info_service::add_contest_to_user,
    },
};

pub fn handle_create_contest(
    deps: &mut DepsMut,
    env: Env,
    contest_info: ContestInfo,
    contest_info_signature_hex: String,
    outcome_id: u8,
    user: Addr,
    amount: Option<Uint128>,
) -> StdResult<Response> {
    // Create the new contest
    create_new_contest(deps, &env, &contest_info, &contest_info_signature_hex)?;
    create_new_contest_bet_summary(deps.storage, &contest_info)?;

    // Place or update a bet for the contest
    let contest_id = contest_info.id();
    place_or_update_bet(deps.storage, &env, &user, &contest_id, &outcome_id, &amount)?;
    // Update the user's contest list
    add_contest_to_user(deps.storage, &user, &contest_id)?;

    // Add the bet to the contest summary
    add_bet_to_contest_summary(deps.storage, &contest_id, &outcome_id, &amount.unwrap())?;

    // Return the execution result
    Ok(
        Response::new().set_data(ExecuteAnswer::CreateContestAnswer {
            status: ResponseStatus::Success,
        }),
    )
}

pub fn handle_bet_on_contest(
    deps: &mut DepsMut,
    env: &Env,
    contest_id: u32,
    outcome_id: u8,
    user: Addr,
    amount_option: Option<Uint128>,
) -> StdResult<Response> {
    place_or_update_bet(
        deps.storage,
        env,
        &user,
        &contest_id,
        &outcome_id,
        &amount_option,
    )?;

    add_contest_to_user(deps.storage, &user, &contest_id)?;

    add_bet_to_contest_summary(
        deps.storage,
        &contest_id,
        &outcome_id,
        &amount_option.unwrap(),
    )?;

    Ok(
        Response::default().set_data(ExecuteAnswer::BetContestAnswer {
            status: ResponseStatus::Success,
        }),
    )
}
pub fn handle_claim(
    deps: &mut DepsMut,
    env: Env,
    contest_id: u32,
    user: Addr,
) -> StdResult<Response> {
    let contest_info = assert_contest_ready_to_be_claimed(deps.storage, &env, &contest_id)?;
    let contest_bet_summary = finalize_contest_outcome(deps, &contest_info)?;
    let claimable_amount = user_claims_bet(deps.storage, &user, &contest_bet_summary)?;

    let snip20 = Snip20::singleton_load(deps.storage)?;
    Ok(Response::default()
        .add_message(snip20.create_send_msg(&user.into_string(), &claimable_amount)?))
}
