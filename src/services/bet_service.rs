use cosmwasm_std::{Addr, Deps, Env, StdError, Storage, Uint128};
use sp_secret_toolkit::macros::identifiable::Identifiable;

use crate::{
    data::{
        bets::{Bet, UserContest, TOTAL_BETS, TOTAL_VOLUME},
        contest_bet_summary::ContestBetSummary,
        contest_info::{ContestId, ContestInfo},
        user_info::{get_users_contest_map, TOTAL_USERS},
    },
    error::bet_error::BetError,
    responses::query::response_types::users_bets::UserContestBetInfo,
};

use super::{
    contest_bet_summary_service::{
        get_contest_bet_summaries, update_contest_bet_summaries_with_results,
    },
    contest_info_service::get_contest_infos_for_ids,
    integrations::price_feed_service::pricefeed::NULL_AND_VOID_CONTEST_RESULT,
    user_info_service::get_unchecked_contests_for_user,
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
            // Update the existing bet amount and save metrics
            bet.add_amount(*amount);
            update_total_volume(storage, amount);
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
            update_total_volume(storage, amount);
            increment_total_bets(storage);
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

pub fn map_to_user_contest_bet_infos(
    filtered_results: Vec<(ContestInfo, ContestBetSummary, Bet)>,
) -> Vec<UserContestBetInfo> {
    let contests_bets: Vec<UserContestBetInfo> = filtered_results
        .into_iter()
        .map(
            |(contest_info, contest_bet_summary, user_bet)| UserContestBetInfo {
                contest_info,
                contest_bet_summary,
                user_bet,
            },
        )
        .collect();
    contests_bets
}

pub fn assert_not_paid(bet: &Bet) -> Result<(), BetError> {
    if bet.has_been_paid() {
        Err(BetError::BetAlreadyPaid)
    } else {
        Ok(())
    }
}

pub fn calculate_user_share(
    contest_bet_summary: &ContestBetSummary,
    bet: &Bet,
) -> Result<Uint128, BetError> {
    // Load state to access fee_percentage
    let fee_percent = contest_bet_summary.get_fee();

    // Calculate the total pool
    let total_pool = contest_bet_summary.calc_total_pool();
    let users_side_bet_allocation = contest_bet_summary.get_allocation(*bet.get_outcome_id())?;

    if total_pool == users_side_bet_allocation {
        // No bets on the other side so return all funds
        return Ok(bet.get_amount().to_owned());
    }

    // Apply the fee
    let mut total_pool_after_fee = total_pool.u128();

    if fee_percent.numerator() > &(0 as u128) {
        total_pool_after_fee = total_pool.u128()
            * (fee_percent.denominator() - fee_percent.numerator())
            / fee_percent.denominator();
    }
    // Get the total allocation for the user's chosen outcome
    let total_allocation_for_outcome = contest_bet_summary.get_allocation(*bet.get_outcome_id())?;

    // Calculate the user's share
    // To avoid floating-point arithmetic, we multiply before dividing
    let user_share =
        bet.get_amount().u128() * total_pool_after_fee / total_allocation_for_outcome.u128();

    Ok(Uint128::from(user_share))
}

pub fn get_users_map_bets(
    deps: Deps,
    env: Env,
    user: Addr,
) -> Result<Vec<(ContestInfo, ContestBetSummary, Bet)>, StdError> {
    let users_contest_ids = get_unchecked_contests_for_user(deps.storage, &user)?;
    let users_contest_infos = get_contest_infos_for_ids(deps.storage, &users_contest_ids)?;
    let mut users_contest_bet_summaries =
        get_contest_bet_summaries(deps.storage, &users_contest_ids)?;
    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        &env,
        &users_contest_infos,
        &mut users_contest_bet_summaries,
    );
    let users_bets = get_bets_for_user_and_contests(deps.storage, &user, &users_contest_ids)?;

    // Filter contests, bet summaries, and bets based on the provided filters
    let filtered_results = filter_claimable(
        &users_contest_infos,
        &users_contest_bet_summaries,
        &users_bets,
    );
    Ok(filtered_results)
}

fn filter_claimable(
    contest_infos: &Vec<ContestInfo>,
    contest_bet_summaries: &Vec<ContestBetSummary>,
    bets: &Vec<Bet>,
) -> Vec<(ContestInfo, ContestBetSummary, Bet)> {
    contest_infos
        .iter()
        .zip(contest_bet_summaries.iter())
        .zip(bets.iter())
        .filter_map(|((contest_info, contest_bet_summary), bet)| {
            if bet.has_been_paid() {
                None
            } else {
                match contest_bet_summary.get_outcome() {
                    Some(outcome) if outcome.get_id() == bet.get_outcome_id() => Some((
                        (*contest_info).clone(),
                        (*contest_bet_summary).clone(),
                        (*bet).clone(),
                    )),
                    _ => None,
                }
            }
        })
        .collect()
}

// Getter function to retrieve the total volume
pub fn get_total_volume(storage: &dyn Storage) -> Uint128 {
    // Load the TOTAL_VOLUME from storage and return it
    let total_volume = TOTAL_VOLUME.load(storage).unwrap_or(Uint128::zero());
    total_volume
}

// Getter function to retrieve the total bets
pub fn get_total_bets(storage: &dyn Storage) -> u64 {
    // Load the TOTAL_BETS from storage and return it
    let total_bets = TOTAL_BETS.load(storage).unwrap_or(0);
    total_bets
}

// Update function for total volume
pub fn update_total_volume(storage: &mut dyn Storage, amount: &Uint128) {
    // Load the current TOTAL_VOLUME
    let current_total_volume: Uint128 = TOTAL_VOLUME.load(storage).unwrap_or(Uint128::zero());

    // Add the command amount to the current total volume
    let updated_total_volume = current_total_volume + amount;

    // Store the updated total volume back into TOTAL_VOLUME
    TOTAL_VOLUME.save(storage, &updated_total_volume).unwrap()
}

// Update function for total bets
pub fn increment_total_bets(storage: &mut dyn Storage) {
    // Load the current TOTAL_BETS
    let current_total_bets = TOTAL_BETS.load(storage).unwrap_or(0);

    // Add the command amount to the current total bets
    let updated_total_bets = current_total_bets + 1;

    // Store the incremented total bets back into TOTAL_BETS
    TOTAL_BETS.save(storage, &updated_total_bets).unwrap()
}

// Update function for total users
pub fn increment_total_users(storage: &mut dyn Storage) {
    // Load the current TOTAL_USERS
    let current_total_users = TOTAL_USERS.load(storage).unwrap_or(0);

    // Add the command amount to the current total users
    let updated_total_users = current_total_users + 1;

    // Store the incremented total bets back into TOTAL_USERS
    TOTAL_USERS.save(storage, &updated_total_users).unwrap()
}

pub fn get_users_number_of_bets(storage: &dyn Storage, address: &Addr) -> u32 {
    get_users_contest_map(address).get_len(storage).unwrap()
}
