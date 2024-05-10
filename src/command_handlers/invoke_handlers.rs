use cosmwasm_std::{DepsMut, Env, Response, StdResult, Uint128};

use crate::{
    msgs::invoke::commands::bet_contest::BetContest,
    responses::execute::{
        execute_response::{ExecuteResponse, ResponseStatus::Success},
        response_types::{bet::BetResonse, create_contest::CreateContestResponse},
    },
    services::{
        bet_service::place_or_update_bet,
        contest_bet_summary_service::{add_bet_to_contest_summary, create_new_contest_bet_summary},
        contest_info_service::{
            assert_outcome_is_on_contest, assert_time_of_close_not_passed, create_new_contest,
            get_contest_info,
        },
        contests_service::add_active_contest,
        state_service::assert_amount_is_greater_than_minimum_bet,
        user_info_service::add_contest_to_user,
    },
};

pub fn handle_create_contest(
    mut deps: DepsMut,
    env: Env,
    command: CreateContest,
    amount_bet: Uint128,
) -> StdResult<Response> {
    // Create the new contest
    let CreateContest {
        contest_info,
        user,
        outcome_id,
        contest_info_signature_hex,
        ..
    } = command;

    assert_time_of_close_not_passed(&contest_info, &env)?;
    create_new_contest(&mut deps, &contest_info)?;
    create_new_contest_bet_summary(deps.storage, &contest_info)?;

    let contest_id = contest_info.get_id();

    add_active_contest(deps.storage, &contest_id)?;

    assert_amount_is_greater_than_minimum_bet(deps.storage, &amount_bet)?;
    assert_outcome_is_on_contest(&contest_info, &outcome_id)?;
    place_or_update_bet(deps.storage, &user, &contest_id, &outcome_id, &amount_bet)?;

    add_contest_to_user(deps.storage, &user, &contest_id)?;
    add_bet_to_contest_summary(deps.storage, &contest_id, &outcome_id, &amount_bet)?;

    Ok(
        Response::new().set_data(ExecuteResponse::CreateContest(CreateContestResponse {
            status: Success,
        })),
    )
}

pub fn handle_bet_on_contest(
    deps: DepsMut,
    env: Env,
    command: BetContest,
    amount_bet: Uint128,
) -> StdResult<Response> {
    let BetContest {
        user,
        contest_id,
        outcome_id,
        ..
    } = command;

    // Load state and assert minimum bet
    assert_amount_is_greater_than_minimum_bet(deps.storage, &amount_bet)?;

    let contest_info = get_contest_info(deps.storage, &contest_id)?;
    assert_outcome_is_on_contest(&contest_info, &outcome_id)?;
    assert_time_of_close_not_passed(&contest_info, &env)?;
    let new_bet = place_or_update_bet(deps.storage, &user, &contest_id, &outcome_id, &amount_bet)?;
    if new_bet {
        add_contest_to_user(deps.storage, &user, &contest_id)?;
    }

    add_bet_to_contest_summary(deps.storage, &contest_id, &outcome_id, &amount_bet)?;

    Ok(Response::default().set_data(ExecuteResponse::Bet(BetResonse { status: Success })))
}
