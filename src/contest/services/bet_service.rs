use cosmwasm_std::{Addr, Env, StdError, StdResult, Storage, Uint128};
use sp_secret_toolkit::macros::identifiable::Identifiable;

use crate::{
    contest::{
        data::{
            bets::{Bet, UserContest},
            contest_bet_summary::ContestBetSummary,
            contest_info::verify_contest,
        },
        error::{bet_error::BetError, contest_info_error::ContestInfoError},
    },
    integrations::oracle::oracle::NULL_AND_VOID_CONTEST_RESULT,
    state::State,
};
// Assuming the existence of State, UserContest, Bet, ContestInfoError, and necessary validation functions.

/// Places a new bet or updates an existing bet for a user on a given contest and outcome.
///
/// # Arguments
///
/// * `deps` - Dependencies for accessing storage, passed by mutable reference.
/// * `env` - The environment data, including block time, passed by reference.
/// * `address` - The address of the user placing the bet, passed by reference.
/// * `contest_id` - The ID of the contest on which the bet is placed, passed by value but it's a Copy type.
/// * `outcome_id` - The ID of the outcome the user is betting on, passed by value but it's a Copy type.
/// * `amount` - The amount of the bet, passed as an Option reference for conditional unpacking.
///
/// # Returns
///
/// Result indicating the operation's success or failure.
pub fn place_or_update_bet(
    storage: &mut dyn Storage,
    env: &Env,
    user: &Addr,
    contest_id: &u32,
    outcome_id: &u8,
    amount_option: &Option<Uint128>, // Borrowing the Option reference
) -> StdResult<()> {
    // Ensure amount is provided
    let amount = amount_option.ok_or_else(|| StdError::generic_err("Amount must be provided"))?;

    // Load state and assert minimum bet
    let state = State::singleton_load(storage)?;
    state.assert_minimum_bet(&amount)?;

    // Verify contest details and timings
    let contest_info = verify_contest(storage, &contest_id, outcome_id.clone())?;
    contest_info.assert_time_of_close_not_passed(env.block.time.seconds())?;

    // Attempt to retrieve an existing bet
    let user_contest_key = UserContest::new(user.clone(), contest_id.clone()); // Cloning address is necessary here for ownership reasons
    match Bet::keymap_get_by_id(storage, &user_contest_key) {
        Some(mut bet) => {
            // If a bet exists but on a different outcome, reject the new bet
            if outcome_id != bet.get_outcome_id() {
                return Err(ContestInfoError::CannotBetOnBothSides.into());
            }
            // Update the existing bet amount and save
            bet.add_amount(amount);
            bet.keymap_save(storage)?;
        }
        None => {
            // If no existing bet, create a new one and save
            let new_bet = Bet::new(
                user.clone(),
                contest_id.clone(),
                amount.clone(),
                outcome_id.clone(),
            ); // Cloning address is necessary for Bet creation
            new_bet.keymap_save(storage)?;
        }
    }

    Ok(())
}

pub fn get_bets_for_user_and_contests(
    storage: &dyn Storage,
    user: &Addr,
    contest_ids: &Vec<u32>,
) -> Result<Vec<Bet>, BetError> {
    let mut bets: Vec<Bet> = Vec::new();

    for contest_id in contest_ids {
        let user_contest_key = UserContest::new(user.clone(), *contest_id);

        match Bet::keymap_get_by_id(storage, &user_contest_key) {
            Some(bet) => bets.push(bet),
            None => {
                // Instantly return an error if a bet is not found for a contest
                return Err(BetError::NoBetForUserContest {
                    user_contest: user_contest_key,
                });
            }
        }
    }

    Ok(bets)
}

pub fn user_claims_bet(
    storage: &mut dyn Storage,
    user: &Addr,
    contest_bet_summary: &ContestBetSummary,
) -> Result<Uint128, BetError> {
    let user_contest = UserContest::new(user.clone(), contest_bet_summary.id());
    let bet_option = Bet::keymap_get_by_id(storage, &user_contest);
    match bet_option {
        Some(mut bet) => {
            bet.assert_not_paid()?;
            let contest_bet_summary_clone = contest_bet_summary.get_outcome().clone().unwrap();
            let winning_outcome_id = contest_bet_summary_clone.get_id();

            let amount_to_claim: Uint128;

            if *winning_outcome_id == NULL_AND_VOID_CONTEST_RESULT {
                // Contest is null and void, return users bet
                amount_to_claim = bet.get_amount().clone();
            } else if bet.get_outcome_id() == winning_outcome_id {
                // User won calculate their payout
                amount_to_claim = contest_bet_summary
                    .calculate_user_share(*bet.get_amount(), *bet.get_outcome_id())?;
            } else {
                // User lost lol
                return Err(BetError::CannotClaimOnLostContest);
            }
            bet.mark_paid();
            bet.keymap_save(storage)?;
            Ok(amount_to_claim)
        }
        None => Err(BetError::NoBetForUserContest { user_contest }.into()),
    }
}

pub fn get_user_bet(storage: &dyn Storage, user_contest: UserContest) -> Result<Bet, BetError> {
    match Bet::keymap_get_by_id(storage, &user_contest) {
        Some(bet) => Ok(bet),
        None => Err(BetError::NoBetForUserContest { user_contest }),
    }
}
