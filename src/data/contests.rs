use cosmwasm_std::{StdResult, Storage};
use secret_toolkit::storage::AppendStore;

use super::contest_info::ContestId;

// Initialize a base AppendStore for contest IDs, assuming contest IDs are stored as String
static CONTEST_ID_STORE: AppendStore<ContestId> = AppendStore::new(b"Contest_Ids");

// Function to get contests AppendStore
pub fn get_all_contest_id_store() -> AppendStore<'static, ContestId> {
    let user_store = CONTEST_ID_STORE.add_suffix(b"all_contests");
    return user_store;
}

pub fn add_contest_id(storage: &mut dyn Storage, contest_id: &ContestId) -> StdResult<()> {
    let contest_store = get_all_contest_id_store();
    return contest_store.push(storage, contest_id);
}
