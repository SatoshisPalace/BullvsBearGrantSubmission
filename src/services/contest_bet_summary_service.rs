use cosmwasm_std::{DepsMut, StdResult, Storage, Uint128};
use sp_secret_toolkit::oracle::response::GetContestResultResponse;

use crate::{
    data::{
        contest_bet_summary::ContestBetSummary,
        contest_info::{ContestInfo, ContestOutcome},
    },
    error::contest_bet_summary_error::ContestBetSummaryError,
};

use super::integrations::oracle_service::oracle::{
    query_contest_result, NULL_AND_VOID_CONTEST_RESULT,
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
    contest_info: &ContestInfo,
) -> Result<ContestBetSummary, ContestBetSummaryError> {
    // Attempt to retrieve the ContestBetSummary from storage.
    let mut contest_bet_summary =
        ContestBetSummary::keymap_get_by_id(deps.storage, &contest_info.get_id())
            .ok_or(ContestBetSummaryError::DNE(contest_info.get_id()))?;

    // Check if an outcome is already set.
    if let Some(_outcome) = contest_bet_summary.get_outcome() {
        return Ok(contest_bet_summary);
    }

    // If not, query the oracle for the contest result.
    let oracle_result: GetContestResultResponse =
        query_contest_result(&deps.querier, deps.storage, &contest_info.get_id())
            .map_err(|_| ContestBetSummaryError::OracleQueryFailed(contest_info.get_id()))?;

    // Handle NULL_AND_VOID_CONTEST_RESULT or find the matching outcome.
    let outcome = if oracle_result.result == NULL_AND_VOID_CONTEST_RESULT {
        ContestOutcome::nullified_result()
    } else {
        contest_info.find_outcome(&oracle_result.result)?
    };

    // Set the outcome in the contest bet summary.
    contest_bet_summary.set_outcome(&outcome)?;

    // Save the updated contest bet summary back to storage.
    contest_bet_summary.keymap_save(deps.storage)?;

    Ok(contest_bet_summary)
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
