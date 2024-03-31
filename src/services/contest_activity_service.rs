use cosmwasm_std::{Env, Storage};

use crate::{
    data::{
        contest_activity::ContestActivity, contest_bet_summary::ContestBetSummary,
        contest_info::ContestInfo,
    },
    error::contest_activity_error::ContestActivityError,
    msgs::query::commands::get_active_contests::ContestQuerySortOrder,
};

use super::{
    contest_bet_summary_service::get_contest_bet_summaries,
    contest_info_service::{assert_time_of_close_not_passed, get_contest_infos_for_ids},
};

pub fn add_active_contest(
    storage: &mut dyn Storage,
    contest_id: &String,
) -> Result<(), ContestActivityError> {
    let mut contest_activity = ContestActivity::singleton_load(storage)?;
    contest_activity.add_contest(contest_id);
    contest_activity.singleton_save(storage)?;

    Ok(())
}

pub fn remove_active_contest(
    storage: &mut dyn Storage,
    contest_id: &String,
) -> Result<(), ContestActivityError> {
    let mut contest_activity = ContestActivity::singleton_load(storage)?;
    contest_activity.remove_contest(contest_id);
    contest_activity.singleton_save(storage)?;

    Ok(())
}

pub fn get_active_contests(
    storage: &dyn Storage,
    env: &Env,
    page_num: Option<u32>,
    page_size: Option<u32>,
    sort_order: Option<ContestQuerySortOrder>,
) -> Result<Vec<(ContestInfo, ContestBetSummary)>, ContestActivityError> {
    let contest_activity = ContestActivity::singleton_load(storage)?;
    let active_contests = contest_activity.get_active_contests_ids();

    let mut contest_infos = get_contest_infos_for_ids(storage, active_contests)?;

    contest_infos.retain(|contest_info| assert_time_of_close_not_passed(contest_info, env).is_ok());

    let contest_ids: Vec<String> = contest_infos
        .iter()
        .map(|info| info.get_id().clone())
        .collect();
    let contest_bet_summaries = get_contest_bet_summaries(storage, &contest_ids)?;

    let mut combined: Vec<(ContestInfo, ContestBetSummary)> = contest_infos
        .into_iter()
        .zip(contest_bet_summaries.into_iter())
        .collect();

    if let Some(ContestQuerySortOrder::Volume) = sort_order {
        combined.sort_by(|a, b| {
            let total_pool_a = a.1.calc_total_pool();
            let total_pool_b = b.1.calc_total_pool();
            total_pool_b.cmp(&total_pool_a) // Assuming Uint128 supports cmp, adjust if necessary
        });
    }

    // Pagination logic as a closure to avoid code duplication
    let paginate = |combined: Vec<(ContestInfo, ContestBetSummary)>,
                    page_num: Option<u32>,
                    page_size: Option<u32>|
     -> Vec<(ContestInfo, ContestBetSummary)> {
        if let (Some(pn), Some(ps)) = (page_num, page_size) {
            combined
                .into_iter()
                .skip((pn as usize) * (ps as usize))
                .take(ps as usize)
                .collect()
        } else {
            combined
        }
    };

    // Apply pagination
    Ok(paginate(combined, page_num, page_size))
}
