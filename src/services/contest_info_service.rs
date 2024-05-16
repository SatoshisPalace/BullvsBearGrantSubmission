// contest_info_service.rs
use cosmwasm_std::{DepsMut, Env, Storage};

use crate::{constants::{BEAR, BULL, FIVE_MINUTE_INTERVAL, SECONDS_IN_A_MINUTE}, data::contest_info::{ContestId, ContestInfo, ContestOutcome}, error::contest_info_error::ContestInfoError};

pub fn create_new_contest(
    deps: &mut DepsMut,
    contest_info: &ContestInfo,
) -> Result<(), ContestInfoError> {
    validate_contest(contest_info)?;

    // Contests cannot be recreated
    let contest_id = contest_info.get_id();
    if ContestInfo::keymap_verify_exists(deps.storage, &contest_id).is_ok() {
        return Err(ContestInfoError::ContestAlreadyExist(contest_id).into());
    }

    contest_info.keymap_save(deps.storage)?;
    Ok(())
}

pub fn get_contest_info(
    storage: &dyn Storage,
    contest_id: &ContestId,
) -> Result<ContestInfo, ContestInfoError> {
    match ContestInfo::keymap_get_by_id(storage, contest_id) {
        Some(contest_info) => Ok(contest_info),
        None => Err(ContestInfoError::ContestNotFound(contest_id.clone())),
    }
}

pub fn get_contest_infos_for_ids(
    storage: &dyn Storage,
    contest_ids: &Vec<ContestId>,
) -> Result<Vec<ContestInfo>, ContestInfoError> {
    let mut contest_infos: Vec<ContestInfo> = Vec::new();

    for contest_id in contest_ids {
        let contest_info = get_contest_info(storage, contest_id)?;
        contest_infos.push(contest_info);
    }

    Ok(contest_infos)
}

pub fn get_contest_infos_for_ids_ignore_missing(
    storage: &dyn Storage,
    contest_ids: &Vec<ContestId>,
) -> Vec<ContestInfo> {
    let mut contest_infos: Vec<ContestInfo> = Vec::new();

    for contest_id in contest_ids {
        match ContestInfo::keymap_get_by_id(storage, contest_id) {
            Some(contest_info) => {
                contest_infos.push(contest_info);
            }
            None => continue,
        }
    }

    return contest_infos;
}

pub fn assert_contest_ready_to_be_claimed(
    storage: &dyn Storage,
    env: &Env,
    contest_id: &ContestId,
) -> Result<ContestInfo, ContestInfoError> {
    let contest_info = get_contest_info(storage, contest_id)?;
    let current_time = env.block.time.seconds();
    if current_time < contest_info.get_time_of_resolve() {
        return Err(ContestInfoError::TimeOfResolveHasYetToPassed {
            contest_id: contest_info.get_id(),
            time_of_resolve: contest_info.get_time_of_resolve(),
            current_time,
        });
    }
    Ok(contest_info)
}

pub fn assert_outcome_is_on_contest(
    contest_info: &ContestInfo,
    outcome_id: &u8,
) -> Result<(), ContestInfoError> {
    if contest_info
        .get_options()
        .iter()
        .any(|outcome| outcome.get_id() == outcome_id)
    {
        Ok(())
    } else {
        Err(ContestInfoError::OutcomeDNE)
    }
}

pub fn assert_time_of_close_not_passed(
    contest_info: &ContestInfo,
    env: &Env,
) -> Result<(), ContestInfoError> {
    let current_time = env.block.time.seconds();
    if current_time >= contest_info.get_time_of_close() {
        Err(ContestInfoError::TimeOfClosePassed(contest_info.get_id()))
    } else {
        Ok(())
    }
}

pub fn assert_time_of_resolved_not_passed(
    contest_info: &ContestInfo,
    env: &Env,
) -> Result<(), ContestInfoError> {
    let current_time = env.block.time.seconds();
    if current_time >= contest_info.get_time_of_resolve() {
        Err(ContestInfoError::TimeOfResolvePassed(contest_info.get_id()))
    } else {
        Ok(())
    }
}

pub fn validate_contest(contest_info: &ContestInfo) -> Result<(), ContestInfoError> {
    if contest_info
        .get_options()
        .iter()
        .any(|outcome| outcome.get_id() == &0u8)
    {
        return Err(ContestInfoError::InvalidOutcomeId {
            contest_id: contest_info.get_id(),
        });
    }
    Ok(())
}

/// Calculates the Unix timestamp for the next 5-minute interval on the minute.
/// This function is used to determine when the next betting round should close,
/// ensuring that all bets are placed within a specific time frame.
///
/// # Arguments
///
/// * `env` - The environment context provided by CosmWasm that includes time information.
///
/// # Returns
///
/// Returns the Unix timestamp of the next 5-minute mark from the current block time.
pub fn get_current_close(env: &Env) -> u64 {
    // Retrieve the current time in seconds from the blockchain's environment.
    let current_seconds = env.block.time.seconds();

    // Calculate the next multiple of the 5-minute interval from the current time.
    // This finds the smallest multiple of FIVE_MINUTE_INTERVAL that is greater than or equal to
    // the current time. The subtraction by 1 and subsequent division and multiplication
    // by FIVE_MINUTE_INTERVAL ensures rounding up to the next interval unless already exactly on one.
    let next_interval_seconds = ((current_seconds + FIVE_MINUTE_INTERVAL - 1) / FIVE_MINUTE_INTERVAL) * FIVE_MINUTE_INTERVAL;

    // Adjust the calculated time to the start of the 5-minute interval.
    // This removes any seconds past the start of the minute, setting the time exactly on the minute mark.
    next_interval_seconds - (next_interval_seconds % SECONDS_IN_A_MINUTE)
}

pub fn create_new_contest_info(ticker: &String, current_close: &u64) -> ContestInfo {
    ContestInfo::new(
        ticker.clone(),
        *current_close, 
        *current_close + FIVE_MINUTE_INTERVAL,
        vec![ContestOutcome::new(1, BULL.to_string()), ContestOutcome::new(2, BEAR.to_string())]
    )
}