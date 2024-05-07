use cosmwasm_std::{DepsMut, Env, QuerierWrapper, StdResult, Storage, Uint128};

use crate::{
    data::{
        contest_bet_summary::ContestBetSummary,
        contest_info::{ContestInfo, ContestOutcome},
    },
    error::contest_bet_summary_error::ContestBetSummaryError,
};

use super::{
    contest_info_service::assert_contest_ready_to_be_claimed,
    integrations::oracle_service::oracle::{query_contest_result, NULL_AND_VOID_CONTEST_RESULT},
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
    contest_id: &String,
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
    // Create a new ContestBetSummary instance
    let contest_bet_summary = ContestBetSummary::new(contest_info);

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

    // If not, query the oracle for the contest result using the adjusted function.
    let oracle_result =
        query_contest_result_oracle(deps.storage, &deps.querier, env, contest_info)?;

    if let Some(outcome) = oracle_result {
        // Should certainly exist
        // Set the outcome in the contest bet summary.
        contest_bet_summary.set_outcome(&outcome)?;
    } else {
        return Err(ContestBetSummaryError::OutcomeDNE);
    }

    // Save the updated contest bet summary back to storage.
    contest_bet_summary.keymap_save(deps.storage)?;

    Ok((contest_bet_summary, true))
}

pub fn get_contest_bet_summary(
    storage: &dyn Storage,
    contest_id: &String,
) -> Result<ContestBetSummary, ContestBetSummaryError> {
    let contest_bet_summary = ContestBetSummary::keymap_get_by_id(storage, contest_id)
        .ok_or(ContestBetSummaryError::DNE(contest_id.clone()))?;
    Ok(contest_bet_summary)
}

pub fn get_contest_bet_summaries_ignore_missing(
    storage: &dyn Storage,
    contest_ids: &Vec<String>,
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

pub fn get_contest_bet_summaries(
    storage: &dyn Storage,
    contest_ids: &Vec<String>,
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
    match query_contest_result(querier, storage, &contest_info.get_id()) {
        Ok(response) => {
            if response.result == NULL_AND_VOID_CONTEST_RESULT {
                Ok(Some(ContestOutcome::nullified_result()))
            } else {
                // Use find_outcome to get the ContestOutcome from the contest_info
                match contest_info.find_outcome(&response.result) {
                    Ok(outcome) => Ok(Some(outcome)),
                    Err(_) => Err(ContestBetSummaryError::OutcomeDNE),
                }
            }
        }
        Err(_) => Ok(None), // Query failed, assume result not available
    }
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

        // Attempt to get the oracle result for the specific contest_info
        if let Ok(Some(outcome)) = query_contest_result_oracle(storage, querier, env, contest_info)
        {
            // Update the contest bet summary with the new outcome
            let _ = contest_bet_summary.set_outcome(&outcome);
        }
        // If the result is not available or the query fails, do not update the summary
    }

    contest_bet_summaries.to_vec() // Return the updated summaries
}
