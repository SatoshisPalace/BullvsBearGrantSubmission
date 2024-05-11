use cosmwasm_std::{StdError, StdResult, Storage};
use secret_toolkit::storage::AppendStore;

use super::contest_info::ContestId;

// Initialize a base AppendStore for contest IDs, assuming contest IDs are stored as String
static CONTEST_ID_STORE: AppendStore<ContestId> = AppendStore::new(b"Contest_Ids");

// Function to get a user-specific AppendStore
fn get_all_contest_id_store() -> AppendStore<'static, ContestId> {
    let user_store = CONTEST_ID_STORE.add_suffix(b"all_contests");
    return user_store;
}

pub fn get_all_contest_ids(storage: &dyn Storage) -> StdResult<Vec<ContestId>> {
    let contest_store = get_all_contest_id_store();
    let result_iter = contest_store.iter(storage)?;
    let results: Vec<Result<ContestId, StdError>> = result_iter.collect();

    // Now, filter out Ok values and handle errors appropriately
    let mut all_contests = Vec::new();
    for result in results {
        match result {
            Ok(contest) => all_contests.push(contest),
            Err(e) => return Err(e.into()), // or handle the error differently
        }
    }

    Ok(all_contests)
}

pub fn add_contest_id(storage: &mut dyn Storage, contest_id: &ContestId) -> StdResult<()> {
    let contest_store = get_all_contest_id_store();
    return contest_store.push(storage, contest_id);
}
