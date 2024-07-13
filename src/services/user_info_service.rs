use cosmwasm_std::{Addr, StdResult, Storage};

use crate::data::{
    contest_info::ContestId,
    user_info::{get_users_contest_map, get_users_last_claimed_index, TOTAL_USERS},
};

use super::bet_service::increment_total_users;

// Function to add a contest ID to a specific user's contest store
pub fn add_contest_to_user(
    storage: &mut dyn Storage,
    address: &Addr,
    contest_id: &ContestId,
) -> StdResult<bool> {
    let users_contests = get_users_contest_map(address);
    let key = users_contests.get_len(storage)?;
    if key == 0 {
        increment_total_users(storage);
    }

    users_contests.insert(storage, &key, contest_id)?;
    Ok(true)
}

// Getter function to retrieve the total users
pub fn get_total_users(storage: &dyn Storage) -> u32 {
    // Load the TOTAL_USERS from storage and return it
    let total_users = TOTAL_USERS.load(storage).unwrap_or(0);
    total_users
}

// Function to advance a users index to the contest id they last claimed
pub fn advance_index(
    storage: &mut dyn Storage,
    address: &Addr,
    contest_id: &ContestId,
) -> StdResult<bool> {
    let user_map = get_users_contest_map(address);
    let user_index = get_users_last_claimed_index(address);
    let search_index = user_index.load(storage).unwrap_or_default();

    for key in search_index..user_map.get_len(storage)?.into() {
        if user_map.get(storage, &key).unwrap() == *contest_id {
            user_index.save(storage, &(key + 1))?;
        }
    }

    Ok(true)
}

pub fn get_last_ten_bet_on(storage: &dyn Storage, address: &Addr) -> StdResult<Vec<ContestId>> {
    let user_map = get_users_contest_map(address);
    let bet_length = user_map.get_len(storage).unwrap();
    let mut range_start = 0;

    if bet_length > 10 {
        range_start = bet_length - 10;
    }

    let mut all_contests = Vec::new();

    for key in range_start..user_map.get_len(storage)?.into() {
        all_contests.push(user_map.get(storage, &key).unwrap())
    }

    Ok(all_contests)
}

pub fn get_users_contest_bets_by_index(
    storage: &dyn Storage,
    list: &Vec<u32>,
    address: &Addr,
) -> StdResult<Vec<ContestId>> {
    let user_map = get_users_contest_map(address);

    let mut all_contests = Vec::new();

    for index in list {
        if let Some(value) = user_map.get(storage, &index) {
            all_contests.push(value);
        }
    }

    Ok(all_contests)
}

// Function to retrieve all contests for a specific user
// Gas Scaling Offender: User's with excessive unclaimed contests will fail
pub fn get_unchecked_contests_for_user(
    storage: &dyn Storage,
    address: &Addr,
) -> StdResult<Vec<ContestId>> {
    let user_map = get_users_contest_map(address);
    let user_index = get_users_last_claimed_index(address)
        .may_load(storage)
        .unwrap();
    let range_start: u32;
    if user_index.is_some() {
        range_start = user_index.unwrap();
    } else {
        range_start = 0;
    }

    // Now, filter out Ok values and handle errors appropriately
    let mut all_contests = Vec::new();

    for key in range_start..user_map.get_len(storage)?.into() {
        all_contests.push(user_map.get(storage, &key).unwrap())
    }

    Ok(all_contests)
}
