use cosmwasm_std::{Addr, Storage, Uint128};
use sp_secret_toolkit::macros::identifiable::Identifiable;

use crate::{
    constants::{FEE_PERCENTAGE, PERCENTAGE_BASE},
    data::{
        bets::{Bet, UserContest},
        contest_bet_summary::ContestBetSummary, contest_info::ContestId,
    },
    error::bet_error::BetError,
};

use super::integrations::price_feed_service::pricefeed::NULL_AND_VOID_CONTEST_RESULT;

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
    user: &Addr,
    contest_id: &ContestId,
    outcome_id: &u8,
    amount: &Uint128, // Borrowing the Option reference
) -> Result<bool, BetError> {
    // Attempt to retrieve an existing bet
    let user_contest_key = UserContest::new(user.clone(), contest_id.clone()); // Cloning address is necessary here for ownership reasons
    match Bet::keymap_get_by_id(storage, &user_contest_key) {
        Some(mut bet) => {
            // If a bet exists but on a different outcome, reject the new bet
            if outcome_id != bet.get_outcome_id() {
                return Err(BetError::CannotBetOnBothSides.into());
            }
            // Update the existing bet amount and save
            bet.add_amount(*amount);
            bet.keymap_save(storage)?;
            Ok(false)
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
            Ok(true)
        }
    }
}

pub fn get_bets_for_user_and_contests(
    storage: &dyn Storage,
    user: &Addr,
    contest_ids: &Vec<ContestId>,
) -> Result<Vec<Bet>, BetError> {
    let mut bets: Vec<Bet> = Vec::new();

    for contest_id in contest_ids {
        let user_contest_key = UserContest::new(user.clone(), contest_id.clone());

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
            assert_not_paid(&bet)?;
            let contest_bet_summary_clone = contest_bet_summary.get_outcome().clone().unwrap();
            let winning_outcome_id = contest_bet_summary_clone.get_id();

            let amount_to_claim: Uint128;

            if *winning_outcome_id == NULL_AND_VOID_CONTEST_RESULT {
                // Contest is null and void, return users bet
                amount_to_claim = bet.get_amount().clone();
            } else if bet.get_outcome_id() == winning_outcome_id {
                // User won calculate their payout
                amount_to_claim = calculate_user_share(contest_bet_summary, &bet)?
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

pub fn assert_not_paid(bet: &Bet) -> Result<(), BetError> {
    if bet.has_been_paid() {
        Err(BetError::BetAlreadyPaid)
    } else {
        Ok(())
    }
}

fn calculate_user_share(
    contest_bet_summary: &ContestBetSummary,
    bet: &Bet,
) -> Result<Uint128, BetError> {
    // Calculate the total pool
    let total_pool = contest_bet_summary.calc_total_pool();
    let users_side_bet_allocation = contest_bet_summary.get_allocation(*bet.get_outcome_id())?;

    if total_pool == users_side_bet_allocation {
        // No bets on the other side so return all funds
        return Ok(bet.get_amount().to_owned());
    }

    // Apply the fee
    let total_pool_after_fee =
        total_pool.u128() * (PERCENTAGE_BASE - FEE_PERCENTAGE) / PERCENTAGE_BASE;

    // Get the total allocation for the user's chosen outcome
    let total_allocation_for_outcome = contest_bet_summary.get_allocation(*bet.get_outcome_id())?;

    // Calculate the user's share
    // To avoid floating-point arithmetic, we multiply before dividing
    let user_share =
        bet.get_amount().u128() * total_pool_after_fee / total_allocation_for_outcome.u128();

    Ok(Uint128::from(user_share))
}
