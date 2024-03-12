use super::{
    data::{
        bets::{get_bet, save_bet, verify_bet, Bet, UserContest},
        contest_bet_summary::{
            get_contest_bet_summary, save_contest_bet_summary, update_contest_bet_summary,
            ContestBetSummary,
        },
        contest_info::{get_contest, save_contest, verify_contest, ContestInfo},
    },
    error::ContestError,
};
use crate::{
    cryptography::cryptography::is_valid_signature,
    integrations::{oracle::constants::NULL_AND_VOID_CONTEST_RESULT, snip_20::snip_20::send},
    state::config_read,
};
use cosmwasm_std::{Addr, DepsMut, Env, Response, StdResult, Uint128};

pub fn try_create_contest<'a>(
    deps: &mut DepsMut,
    env: &Env,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), ContestError> {
    contest_info.assert_time_of_close_not_passed(env.block.time.seconds())?;
    contest_info.validate_contest()?;

    let state = config_read(deps.storage).load()?;
    //Validate Signature
    let contest_info_json: String = contest_info.to_json();
    is_valid_signature(
        deps.api,
        state.satoshis_palace.as_str(),
        &contest_info_json,
        &contest_info_signature_hex,
    )?;

    // Contests cannot be recreated
    let contest_id = contest_info.id();
    if let Some(_) = get_contest(deps.storage, contest_id) {
        return Err(ContestError::ContestAlreadyExist(contest_id));
    }

    save_contest(deps.storage, &contest_info);

    // Initialize and save the ContestBetSummary
    let contest_bet_summary = ContestBetSummary::initialize(contest_info);
    save_contest_bet_summary(deps.storage, contest_info.id, &contest_bet_summary)?;

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
    verify_bet(&Some(user.clone()), amount)?;
    let contest_info = verify_contest(deps.storage, contest_id, outcome_id)?;
    contest_info.assert_time_of_close_not_passed(env.block.time.seconds())?;

    let user_contest = UserContest {
        address: user.clone(),
        contest_id,
    };
    let bet = get_bet(deps.storage, &user_contest);
    match bet {
        Some(bet) => {
            // User has bet before
            if outcome_id != bet.outcome_id {
                return Err(ContestError::CannotBetOnBothSides);
            }
            save_bet(
                deps.storage,
                user.clone(),
                contest_id,
                amount.unwrap() + bet.amount,
                outcome_id,
                false,
            )?;
        }
        None => save_bet(
            // User has not bet before
            deps.storage,
            user,
            contest_id,
            amount.unwrap(),
            outcome_id,
            false,
        )?,
    }

    update_contest_bet_summary(deps.storage, contest_id, amount.unwrap(), outcome_id)?;
    Ok(())
}

pub fn try_claim(
    deps: &mut DepsMut,
    env: &Env,
    contest_id: u32,
    sender: Addr,
) -> StdResult<Response> {
    let contest_info: ContestInfo = get_contest(deps.storage, contest_id).unwrap();
    contest_info.assert_time_of_resolve_is_passed(env.block.time.seconds())?;
    let mut contest_bet_summary: ContestBetSummary =
        get_contest_bet_summary(deps.storage, contest_id).unwrap();
    let contest_result =
        contest_bet_summary.get_outcome(&deps.querier, deps.storage, &contest_info, contest_id)?;
    save_contest_bet_summary(deps.storage, contest_info.id, &contest_bet_summary)?;

    // Create a UserContest instance
    let user_contest = UserContest {
        address: sender.clone(),
        contest_id,
    };

    // Check if there is a bet for the given UserContest
    let bet_option = get_bet(deps.storage, &user_contest);

    match bet_option {
        Some(bet) => {
            bet.assert_not_paid()?;

            // Check if contest result is NULL_AND_VOID_CONTEST_RESULT
            if contest_result.id == NULL_AND_VOID_CONTEST_RESULT {
                return handle_null_and_void_contest(deps, sender, contest_id, bet);
            }

            // Check if user's outcome_id matches the contest result
            if bet.outcome_id == contest_result.id {
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
    bet: Bet,
) -> StdResult<Response> {
    let contest_bet_summary_option = get_contest_bet_summary(deps.storage, contest_id);
    match contest_bet_summary_option {
        Some(contest_bet_summary) => {
            // Calculate the user's share
            let user_share =
                contest_bet_summary.calculate_user_share(bet.amount, bet.outcome_id)?;
            save_bet(
                deps.storage,
                sender.clone(),
                contest_id,
                bet.amount,
                bet.outcome_id,
                true,
            )?;
            return send(deps, sender.to_string(), user_share);
        }
        None => Err(ContestError::ContestNotFound(contest_id).into()),
    }
}

fn handle_null_and_void_contest(
    deps: &mut DepsMut,
    sender: Addr,
    contest_id: u32,
    bet: Bet,
) -> StdResult<Response> {
    // Since the contest is null and void, we simply refund the full original bet

    // Update the bet as paid to prevent duplicate refunds
    save_bet(
        deps.storage,
        sender.clone(),
        contest_id,
        bet.amount,
        bet.outcome_id,
        true,
    )?;

    // Use the send function to refund the full original bet
    return send(deps, sender.to_string(), bet.amount.into());
}
