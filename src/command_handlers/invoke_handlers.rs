use cosmwasm_std::{DepsMut, Env, Response, StdResult, Uint128};

use crate::{
    data::contest_info::ContestId, msgs::invoke::commands::bet_contest::BetContest, responses::execute::{
        execute_response::{ExecuteResponse, ResponseStatus::Success},
        response_types::bet::BetResonse,
    }, services::{
        bet_service::place_or_update_bet,
        contest_bet_summary_service::{add_bet_to_contest_summary, create_new_contest_bet_summary},
        contest_info_service::{
            assert_outcome_is_on_contest, assert_ticker_valid, create_new_contest, create_new_contest_info, get_contest_info, get_current_close
        },
        contests_service::add_active_contest,
        state_service::assert_amount_is_greater_than_minimum_bet,
        user_info_service::add_contest_to_user,
    }
};

pub fn handle_bet_on_contest(
    mut deps: DepsMut,
    env: Env,
    command: BetContest,
    amount_bet: Uint128,
) -> StdResult<Response> {
    let BetContest {
        ticker,
        outcome_id,
        user,
        ..
    } = command;

    // Load state and assert minimum bet
    assert_amount_is_greater_than_minimum_bet(deps.storage, &amount_bet)?;

    // Generate current close time
    let current_close = get_current_close(deps.storage, &env);
    // Generate ContestId from ticker and close time
    let contest_id = ContestId::new(ticker.clone(), current_close);
    // Attempt to load contest info
    let contest_info_result = get_contest_info(deps.storage, &contest_id);

    // Handle the case where the contest does not exist
    let contest_info = match contest_info_result {
        Ok(info) => info,
        Err(_e) => {
            assert_ticker_valid(&ticker)?;
            // Initialize new ContestInfo here if needed
            let info = create_new_contest_info(deps.storage, &ticker, &current_close);
            create_new_contest(&mut deps, &info)?;
            create_new_contest_bet_summary(deps.storage, &info)?;
            add_active_contest(deps.storage, &contest_id)?;

            info
        }
    };

    assert_outcome_is_on_contest(&contest_info, &outcome_id)?;
    let new_bet = place_or_update_bet(deps.storage, &user, &contest_id, &outcome_id, &amount_bet)?;
    if new_bet {
        add_contest_to_user(deps.storage, &user, &contest_id)?;
    }

    add_bet_to_contest_summary(deps.storage, &contest_id, &outcome_id, &amount_bet)?;

    Ok(Response::default().set_data(ExecuteResponse::Bet(BetResonse { status: Success })))
}
