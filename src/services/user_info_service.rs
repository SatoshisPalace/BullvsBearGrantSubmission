use cosmwasm_std::{Addr, StdError, StdResult, Storage};

use crate::data::{contest_info::ContestId, user_info::get_user_contest_store};

// Function to add a contest ID to a specific user's contest store
pub fn add_contest_to_user(
    storage: &mut dyn Storage,
    address: &Addr,
    contest_id: &ContestId,
) -> StdResult<bool> {
    let user_store = get_user_contest_store(address);

    // Check if the contest ID already exists (omitted here, assuming it's handled elsewhere)
    // For now, assuming the check is done or not required, directly push the contest ID
    user_store.push(storage, contest_id)?;

    Ok(true)
}

// Function to retrieve all contests for a specific user
pub fn get_contests_for_user(storage: &dyn Storage, address: &Addr) -> StdResult<Vec<ContestId>> {
    let user_store = get_user_contest_store(address);
    let result_iter = user_store.iter(storage)?;

    // Collect results and errors
    let results: Vec<Result<ContestId, StdError>> = result_iter.collect();

    // Now, filter out Ok values and handle errors appropriately
    let mut all_contests = Vec::new();
    for result in results {
        match result {
            Ok(contest) => all_contests.push(contest),
            Err(e) => return Err(e), // or handle the error differently
        }
    }

    Ok(all_contests)
}
