use cosmwasm_std::{Addr, StdResult, Storage};

use crate::{data::user_info::UserInfo, error::user_info_error::UserInfoError};

// Assuming `address` and `contest_id` are properly defined and passed to this function,
// along with `deps` which includes storage access.
pub fn add_contest_to_user(
    storage: &mut dyn Storage,
    address: &Addr,
    contest_id: &String,
) -> StdResult<bool> {
    // Attempt to retrieve the user from storage by address
    let mut user = match UserInfo::keymap_get_by_id(storage, address) {
        Some(user) => user,
        None => {
            // If the user doesn't exist, create a new one
            UserInfo::new(address)
        }
    };

    // Add the contest ID to the user's contests
    let is_new_contest_for_user = user.add_contest(contest_id.to_owned());

    // Save the updated or new user back to storage
    user.keymap_save(storage)?;

    Ok(is_new_contest_for_user)
}

// New function to retrieve contests for a user
pub fn get_contests_for_user(storage: &dyn Storage, address: &Addr) -> StdResult<Vec<String>> {
    match UserInfo::keymap_get_by_id(storage, address) {
        Some(user) => Ok(user.get_contests()),
        None => Err(UserInfoError::UserInfoNotFound(address.to_string()).into()),
    }
}
