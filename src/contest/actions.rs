use super::{
    data::{
        bets::{Bet, UserContest},
        contest_bet_summary::ContestBetSummary,
        contest_info::{verify_contest, ContestInfo},
    },
    error::ContestError,
};
use crate::{
    cryptography::cryptography::is_valid_signature,
    integrations::oracle::constants::NULL_AND_VOID_CONTEST_RESULT, state::State,
};
use cosmwasm_std::{Addr, DepsMut, Env, Response, StdResult, Uint128};
use sp_secret_toolkit::snip20::Snip20;

pub fn try_create_contest<'a>(
    deps: &mut DepsMut,
    env: &Env,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), ContestError> {
    contest_info.assert_time_of_close_not_passed(env.block.time.seconds())?;
    contest_info.validate_contest()?;

    let state = State::singleton_load(deps.storage)?;
    //Validate Signature
    let contest_info_json: String = contest_info.to_json();
    is_valid_signature(
        deps.api,
        state.get_satoshis_palace_signing_address().as_str(),
        &contest_info_json,
        &contest_info_signature_hex,
    )?;

    // Contests cannot be recreated
    let contest_id = contest_info.id();
    if ContestInfo::keymap_verify_exists(deps.storage, &contest_id).is_ok() {
        return Err(ContestError::ContestAlreadyExist(contest_id));
    }

    contest_info.keymap_save(deps.storage)?;

    // Initialize and save the ContestBetSummary
    let contest_bet_summary = ContestBetSummary::initialize(contest_info);
    contest_bet_summary.keymap_save(deps.storage)?;

    Ok(())
}

pub fn try_bet_on_contest(
    deps: &mut DepsMut,
    env: &Env,
    contest_id: u32,
    outcome_id: u8,
    user: Addr,
    amount: Option<Uint128>,
) -> Result<(), ContestError> {
    let state = State::singleton_load(deps.storage)?;
    state.assert_minimum_bet(&amount.unwrap())?;

    let contest_info = verify_contest(deps.storage, &contest_id, outcome_id)?;
    contest_info.assert_time_of_close_not_passed(env.block.time.seconds())?;

    let user_contest = UserContest::new(user.clone(), contest_id);

    let bet = Bet::keymap_get_by_id(deps.storage, &user_contest);
    match bet {
        Some(mut bet) => {
            // User has bet before
            if outcome_id != bet.get_outcome_id().to_owned() {
                return Err(ContestError::CannotBetOnBothSides);
            }
            bet.add_amount(amount.unwrap());
            bet.keymap_save(deps.storage)?;
        }
        None => {
            let bet = Bet::new(user, contest_id, amount.unwrap(), outcome_id);
            bet.keymap_save(deps.storage)?;
        }
    }
    let mut contest_bet_summary =
        ContestBetSummary::keymap_get_by_id(deps.storage, &contest_id).unwrap();
    contest_bet_summary.add_bet_to_option(outcome_id, amount.unwrap())?;
    contest_bet_summary.keymap_save(deps.storage)?;
    Ok(())
}

pub fn try_claim(
    deps: &mut DepsMut,
    env: &Env,
    contest_id: u32,
    sender: Addr,
) -> StdResult<Response> {
    let contest_info: ContestInfo =
        ContestInfo::keymap_get_by_id(deps.storage, &contest_id).unwrap();
    contest_info.assert_time_of_resolve_is_passed(env.block.time.seconds())?;
    let mut contest_bet_summary: ContestBetSummary =
        ContestBetSummary::keymap_get_by_id(deps.storage, &contest_id).unwrap();

    let contest_result = contest_bet_summary.query_set_outcome(
        &deps.querier,
        deps.storage,
        &contest_info,
        contest_id,
    )?;
    contest_bet_summary.keymap_save(deps.storage)?;

    // Create a UserContest instance
    let user_contest = UserContest::new(sender.clone(), contest_id);

    // Check if there is a bet for the given UserContest
    let bet_option = Bet::keymap_get_by_id(deps.storage, &user_contest);

    match bet_option {
        Some(bet) => {
            bet.assert_not_paid()?;

            // Check if contest result is NULL_AND_VOID_CONTEST_RESULT
            if contest_result.id == NULL_AND_VOID_CONTEST_RESULT {
                return handle_null_and_void_contest(deps, sender, bet);
            }

            // Check if user's outcome_id matches the contest result
            if bet.get_outcome_id().to_owned() == contest_result.id {
                return handle_winning_claim(deps, sender, contest_id, bet);
            } else {
                // User's Lost Bet
                Err(ContestError::CannotClaimOnLostContest.into())
            }
        }
        None => Err(ContestError::NoBetForUserContest { user_contest }.into()),
    }
}

// This function contains the logic to handle a successful bet.
fn handle_winning_claim(
    deps: &mut DepsMut,
    sender: Addr,
    contest_id: u32,
    mut bet: Bet,
) -> StdResult<Response> {
    let contest_bet_summary_option = ContestBetSummary::keymap_get_by_id(deps.storage, &contest_id);
    match contest_bet_summary_option {
        Some(contest_bet_summary) => {
            // Calculate the user's share
            let user_share = contest_bet_summary.calculate_user_share(
                bet.get_amount().to_owned(),
                bet.get_outcome_id().to_owned(),
            )?;
            bet.mark_paid();
            bet.keymap_save(deps.storage)?;
            let snip20 = Snip20::singleton_load(deps.storage)?;
            Ok(Response::default()
                .add_message(snip20.create_send_msg(&sender.to_string(), &user_share)?))
        }
        None => Err(ContestError::ContestNotFound(contest_id).into()),
    }
}

fn handle_null_and_void_contest(
    deps: &mut DepsMut,
    sender: Addr,
    mut bet: Bet,
) -> StdResult<Response> {
    // Since the contest is null and void, we simply refund the full original bet
    bet.mark_paid();
    bet.keymap_save(deps.storage)?;

    let snip20 = Snip20::singleton_load(deps.storage)?;
    Ok(Response::default()
        .add_message(snip20.create_send_msg(&sender.to_string(), bet.get_amount())?))
}
