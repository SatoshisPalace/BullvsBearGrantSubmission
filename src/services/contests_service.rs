use cosmwasm_std::{Deps, Storage};

use crate::{
    data::{
        contest_info::{ContestId, ContestInfo},
        contests::{add_contest_id, get_all_contest_id_store},
    },
    error::contest_activity_error::ContestActivityError,
};

use super::integrations::price_feed_service::pricefeed::query_prices;

pub fn add_active_contest(
    storage: &mut dyn Storage,
    contest_id: &ContestId,
) -> Result<(), ContestActivityError> {
    add_contest_id(storage, contest_id)?;
    Ok(())
}

pub fn get_times_to_resolve_from_contest_infos(
    deps: &Deps,
    contest_infos: Vec<ContestInfo>,
) -> Vec<u64> {
    let mut times = vec![];
    let mut close: u64;
    let mut resolve: u64;
    for contest_info in contest_infos {
        close = contest_info.get_time_of_close();
        resolve = contest_info.get_time_of_resolve();

        if query_prices(&deps.querier, deps.storage, &vec![close])
            .unwrap()
            .prices
            .is_empty()
            & !times.contains(&close)
        {
            times.push(close)
        }
        if query_prices(&deps.querier, deps.storage, &vec![resolve])
            .unwrap()
            .prices
            .is_empty()
            & !times.contains(&resolve)
        {
            times.push(resolve)
        }
    }

    times
}

pub fn get_last_ten_contest_ids(storage: &dyn Storage) -> Vec<ContestId> {
    let contest_store = get_all_contest_id_store();
    let store_length = contest_store.get_len(storage).unwrap();
    let mut contest_ids: Vec<ContestId> = vec![];
    let mut start_index: u32 = 0;

    if store_length >= 10 {
        start_index = store_length - 10;
    }

    for key in start_index..store_length {
        contest_ids.push(contest_store.get_at(storage, key).unwrap())
    }

    contest_ids
}

pub fn get_total_number_of_contests(storage: &dyn Storage) -> u32 {
    let contests = get_all_contest_id_store();
    contests.get_len(storage).unwrap()
}
