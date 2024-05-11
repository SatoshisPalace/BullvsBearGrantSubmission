use cosmwasm_std::Addr;
use secret_toolkit::storage::AppendStore;

use super::contest_info::ContestId;

// Initialize a base AppendStore for contest IDs, assuming contest IDs are stored as String
static USER_CONTESTS_STORE: AppendStore<ContestId> = AppendStore::new(b"users_contests");

// Function to get a user-specific AppendStore
pub fn get_user_contest_store(user: &Addr) -> AppendStore<ContestId> {
    let user_store = USER_CONTESTS_STORE.add_suffix(user.as_bytes());
    return user_store;
}
