use cosmwasm_std::{DepsMut, Env, QuerierWrapper, StdResult, Storage, Uint128};

use crate::{
    constants::EXPIRATION_WINDOW,
    data::{
        contest_bet_summary::ContestBetSummary,
        contest_info::{ContestId, ContestInfo, ContestOutcome},
        state::State,
    },
    error::contest_bet_summary_error::ContestBetSummaryError,
};

use super::{
    contest_info_service::{assert_contest_ready_to_be_claimed, get_contest_result},
    integrations::price_feed_service::pricefeed::{query_prices, NULL_AND_VOID_CONTEST_RESULT},
    state_service::add_claimable_fee_for_pool,
}; // Make sure to adjust the import based on your actual storage handling

/// Adds a bet to a contest summary.
///
/// # Arguments
///
/// * `deps` - A mutable reference to the contract's dependencies, including storage.
/// * `contest_id` - The ID of the contest to add the bet to.
/// * `outcome_id` - The ID of the outcome the bet is placed on.
/// * `amount` - The amount of the bet.
///
/// # Returns
///
/// A result indicating success or failure.
pub fn add_bet_to_contest_summary(
    storage: &mut dyn Storage,
    contest_id: &ContestId,
    outcome_id: &u8,
    amount: &Uint128, // Adjust the type according to your contract's definition
) -> StdResult<()> {
    // Attempt to retrieve the ContestBetSummary from storage.
    // Adjust the method call according to your storage handling.
    let mut contest_bet_summary =
        ContestBetSummary::keymap_get_by_id(storage, &contest_id).unwrap();

    // Add the bet to the specified option.
    contest_bet_summary.add_bet_to_option(outcome_id, amount)?;

    // Save the updated summary back to storage.
    // Adjust the method call according to your storage handling.
    contest_bet_summary.keymap_save(storage)?;

    Ok(())
}

/// Creates a new ContestBetSummary and saves it to storage.
///
/// # Arguments
///
/// * `deps` - A mutable reference to the dependencies, including storage access.
/// * `contest_info` - Information about the contest to create a summary for. Borrowed parameter.
///
/// # Returns
///
/// Result indicating the operation's success or failure.
pub fn create_new_contest_bet_summary(
    storage: &mut dyn Storage,
    contest_info: &ContestInfo, // Borrowing contest_info
) -> StdResult<()> {
    let state = State::singleton_load(storage).unwrap();
    let fee = state.fee_percent();

    // Create a new ContestBetSummary instance
    let contest_bet_summary = ContestBetSummary::new(contest_info, fee);

    // Save the ContestBetSummary instance to storage
    contest_bet_summary.keymap_save(storage)?;

    Ok(())
}

pub fn finalize_contest_outcome(
    deps: &mut DepsMut,
    env: &Env,
    contest_info: &ContestInfo,
) -> Result<(ContestBetSummary, bool), ContestBetSummaryError> {
    // Attempt to retrieve the ContestBetSummary from storage.
    let mut contest_bet_summary =
        ContestBetSummary::keymap_get_by_id(deps.storage, &contest_info.get_id())
            .ok_or(ContestBetSummaryError::DNE(contest_info.get_id()))?;

    // Check if an outcome is already set.
    if let Some(_outcome) = contest_bet_summary.get_outcome() {
        return Ok((contest_bet_summary, false));
    }

    // If not and not passed expiration time window, query the price feed for the prices using the adjusted function.
    let price_posting_ids = vec![
        contest_info.get_time_of_close(),
        contest_info.get_time_of_resolve(),
    ];
    let prices = query_prices(&deps.querier, deps.storage, &price_posting_ids);
    let expiry = EXPIRATION_WINDOW + contest_info.get_time_of_close();
    let result: Option<ContestOutcome>;

    // if one side does not have a bet set to null and void
    if bets_on_both_sides(contest_bet_summary.clone()) {
        result = get_contest_result(env, &prices, &expiry);
    } else {
        result = Some(ContestOutcome::nullified_result())
    }

    if let Some(outcome) = result {
        // Should certainly exist
        // Set the outcome in the contest bet summary.

        contest_bet_summary.set_outcome(&outcome)?;
        if outcome.get_id() != &NULL_AND_VOID_CONTEST_RESULT {
            take_contest_fees(deps.storage, contest_bet_summary.clone());
        }
    } else {
        return Err(ContestBetSummaryError::OutcomeDNE);
    }

    // Save the updated contest bet summary back to storage.
    contest_bet_summary.keymap_save(deps.storage)?;

    Ok((contest_bet_summary, true))
}

pub fn get_contest_bet_summary(
    storage: &dyn Storage,
    contest_id: &ContestId,
) -> Result<ContestBetSummary, ContestBetSummaryError> {
    let contest_bet_summary = ContestBetSummary::keymap_get_by_id(storage, contest_id)
        .ok_or(ContestBetSummaryError::DNE(contest_id.clone()))?;
    Ok(contest_bet_summary)
}

pub fn get_contest_bet_summaries_ignore_missing(
    storage: &dyn Storage,
    contest_ids: &Vec<ContestId>,
) -> Vec<ContestBetSummary> {
    let mut contest_bet_summaries: Vec<ContestBetSummary> = Vec::new();

    for contest_id in contest_ids {
        match ContestBetSummary::keymap_get_by_id(storage, contest_id) {
            Some(contest_bet_summary) => {
                contest_bet_summaries.push(contest_bet_summary);
            }
            None => continue,
        }
    }

    return contest_bet_summaries;
}

pub fn bets_on_both_sides(contest_bet_summary: ContestBetSummary) -> bool {
    if !(contest_bet_summary.get_allocation(1).unwrap() == Uint128::zero()
        || contest_bet_summary.get_allocation(2).unwrap() == Uint128::zero())
    {
        return true;
    } else {
        return false;
    }
}

pub fn get_contest_bet_summaries(
    storage: &dyn Storage,
    contest_ids: &Vec<ContestId>,
) -> Result<Vec<ContestBetSummary>, ContestBetSummaryError> {
    let mut contest_bet_summaries: Vec<ContestBetSummary> = Vec::new();

    for contest_id in contest_ids {
        match ContestBetSummary::keymap_get_by_id(storage, contest_id) {
            Some(contest_bet_summary) => {
                contest_bet_summaries.push(contest_bet_summary);
            }
            None => return Err(ContestBetSummaryError::DNE(contest_id.to_owned())),
        }
    }

    return Ok(contest_bet_summaries);
}

pub fn query_contest_result_oracle(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
    contest_info: &ContestInfo,
) -> Result<Option<ContestOutcome>, ContestBetSummaryError> {
    assert_contest_ready_to_be_claimed(storage, env, &contest_info.get_id())?;
    let prices = query_prices(
        querier,
        storage,
        &vec![
            contest_info.get_time_of_close(),
            contest_info.get_time_of_resolve(),
        ],
    );
    let expiry = EXPIRATION_WINDOW + contest_info.get_time_of_close();
    Ok(get_contest_result(env, &prices, &expiry))
}

pub fn update_contest_bet_summaries_with_results(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    env: &Env,
    contest_infos: &Vec<ContestInfo>, // Added vector of ContestInfos
    contest_bet_summaries: &mut Vec<ContestBetSummary>,
) -> Vec<ContestBetSummary> {
    for (contest_info, contest_bet_summary) in
        contest_infos.iter().zip(contest_bet_summaries.iter_mut())
    {
        // Check if an outcome is already set for this summary
        if contest_bet_summary.get_outcome().is_some() {
            continue; // Skip if already set
        }
        // Only check results if its not meant to be null from lack of participation
        if bets_on_both_sides(contest_bet_summary.clone()) {
            // Attempt to get the oracle result for the specific contest_info
            if let Ok(Some(outcome)) =
                query_contest_result_oracle(storage, querier, env, contest_info)
            {
                // Update the contest bet summary with the new outcome
                let _ = contest_bet_summary.set_outcome(&outcome);
            }
        } else {
            let _ = contest_bet_summary.set_outcome(&ContestOutcome::nullified_result());
        }
        // If the result is not available or the query fails, do not update the summary
    }

    contest_bet_summaries.to_vec() // Return the updated summaries
}

pub fn take_contest_fees(storage: &mut dyn Storage, contest_bet_summary: ContestBetSummary) {
    let total_pool = contest_bet_summary.calc_total_pool();
    let fee = contest_bet_summary.get_fee();
    add_claimable_fee_for_pool(storage, &total_pool, &fee);
}
