// contest_info_service.rs
use cosmwasm_std::{DepsMut, Env, StdError, Storage};
use sp_secret_toolkit::price_feed::response::response_types::prices_by_ids::PricesByIdsResponse;

use crate::services::state_service::get_interval;
use crate::{
    constants::{BEAR, BULL, SECONDS_IN_A_MINUTE, TICKERS},
    data::contest_info::{ContestId, ContestInfo, ContestOutcome},
    error::contest_info_error::ContestInfoError,
};
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

pub fn assert_ticker_valid(ticker: &String) -> Result<(), ContestInfoError> {
    if TICKERS.contains(&ticker.as_str()) {
        Ok(())
    } else {
        Err(ContestInfoError::InvalidTicker)
    }
}

pub fn get_contest_result(
    env: &Env,
    prices: &Result<PricesByIdsResponse, StdError>,
    expiry: &u64,
) -> Option<ContestOutcome> {
    // Handle the case where prices contain an error
    let prices = match prices {
        Ok(prices) => &prices.prices,
        Err(_) => {
            if expiry < &env.block.time.seconds() {
                return Some(ContestOutcome::nullified_result());
            } else {
                return None;
            }
        }
    };

    // Ensure there are exactly two price postings and handle if time has passed
    if prices.len() != 2 {
        if expiry < &env.block.time.seconds() {
            return Some(ContestOutcome::nullified_result());
        }
        return None;
    }
    // Compare the first and second price postings
    if prices[0].price() < prices[1].price() {
        Some(ContestOutcome::new(1, "First price is greater".to_string()))
    } else if prices[0].price() > prices[1].price() {
        Some(ContestOutcome::new(
            2,
            "Second price is greater".to_string(),
        ))
    } else {
        Some(ContestOutcome::nullified_result())
    }
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
pub fn get_current_close(storage: &dyn Storage, env: &Env) -> u64 {
    // Retrieve the current time in seconds from the blockchain's environment.
    let current_seconds = env.block.time.seconds();
    let interval = get_interval(storage).unwrap();

    // Calculate the next multiple of the 5-minute interval from the current time.
    // This finds the smallest multiple of FIVE_MINUTE_INTERVAL that is greater than or equal to
    // the current time. The subtraction by 1 and subsequent division and multiplication
    // by FIVE_MINUTE_INTERVAL ensures rounding up to the next interval unless already exactly on one.
    let next_interval_seconds = ((current_seconds + interval - 1) / interval) * interval;

    // Adjust the calculated time to the start of the 5-minute interval.
    // This removes any seconds past the start of the minute, setting the time exactly on the minute mark.
    next_interval_seconds - (next_interval_seconds % SECONDS_IN_A_MINUTE)
}

pub fn create_new_contest_info(
    storage: &dyn Storage,
    ticker: &String,
    current_close: &u64,
) -> ContestInfo {
    ContestInfo::new(
        ticker.clone(),
        *current_close,
        *current_close + get_interval(storage).unwrap(),
        vec![
            ContestOutcome::new(1, BULL.to_string()),
            ContestOutcome::new(2, BEAR.to_string()),
        ],
    )
}
