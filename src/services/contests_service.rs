use cosmwasm_std::{Deps, Env, Storage};

use crate::{
    data::{
        contest_bet_summary::ContestBetSummary,
        contest_info::{ContestId, ContestInfo},
        contests::{add_contest_id, get_all_contest_ids},
    }, error::contest_activity_error::ContestActivityError, msgs::query::commands::get_contests::{ContestQueryFilter, ContestQuerySortOrder}
};

use super::{
    contest_bet_summary_service::{
        get_contest_bet_summaries, update_contest_bet_summaries_with_results,
    },
    contest_info_service::{
        assert_time_of_close_not_passed, assert_time_of_resolved_not_passed,
        get_contest_infos_for_ids,
    },
};

pub fn add_active_contest(
    storage: &mut dyn Storage,
    contest_id: &ContestId,
) -> Result<(), ContestActivityError> {
    add_contest_id(storage, contest_id)?;
    Ok(())
}

fn paginate_contests(
    combined: Vec<(ContestInfo, ContestBetSummary)>,
    page_num: Option<u32>,
    page_size: Option<u32>,
) -> Vec<(ContestInfo, ContestBetSummary)> {
    if let (Some(pn), Some(ps)) = (page_num, page_size) {
        combined
            .into_iter()
            .skip((pn as usize) * (ps as usize))
            .take(ps as usize)
            .collect()
    } else {
        combined
    }
}

pub fn sort_contests(combined: &mut Vec<(ContestInfo, ContestBetSummary)>, sort_order: Option<ContestQuerySortOrder>) {
    if let Some(ContestQuerySortOrder::Volume) = sort_order {
        combined.sort_by(|a, b| {
            let total_pool_a = a.1.calc_total_pool();
            let total_pool_b = b.1.calc_total_pool();
            total_pool_b.cmp(&total_pool_a) // Assuming Uint128 supports cmp, adjust if necessary
        });
    }
}


fn apply_filters(
    combined: &mut Vec<(ContestInfo, ContestBetSummary)>,
    filter: Option<ContestQueryFilter>,
    env: &Env,
) {
    match filter {
        Some(ContestQueryFilter::Active) => {
            combined.retain(|(contest_info, _)| {
                assert_time_of_close_not_passed(contest_info, env).is_ok()
            });
        }
        Some(ContestQueryFilter::Unresolved) => {
            combined.retain(|(contest_info, contest_bet_summary)| {
                // Check if time of close has passed
                if assert_time_of_close_not_passed(contest_info, env).is_ok() {
                    return false;
                }
                // Check if time of resolution has not passed
                if let Err(_) = assert_time_of_resolved_not_passed(contest_info, env) {
                    return false;
                }
                // Check if contest outcome is None
                if contest_bet_summary.get_outcome().is_some() {
                    return false;
                }
                true
            });
        }
        _ => {} // No filter specified, do nothing
    }
}

pub fn get_contests(
    deps: &Deps,
    env: &Env,
    page_num: Option<u32>,
    page_size: Option<u32>,
    sort_order: Option<ContestQuerySortOrder>,
    filter: Option<ContestQueryFilter>,
) -> Result<Vec<(ContestInfo, ContestBetSummary)>, ContestActivityError> {
    let all_contests = get_all_contest_ids(deps.storage)?;

    let contest_infos = get_contest_infos_for_ids(deps.storage, &all_contests)?;

    let contest_ids: Vec<ContestId> = contest_infos
        .iter()
        .map(|info| info.get_id().clone())
        .collect();
    let mut contest_bet_summaries = get_contest_bet_summaries(deps.storage, &contest_ids)?;

    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        env,
        &contest_infos,
        &mut contest_bet_summaries,
    );

    let mut combined: Vec<(ContestInfo, ContestBetSummary)> = contest_infos
        .into_iter()
        .zip(contest_bet_summaries.into_iter())
        .collect();


    // Apply filters based on the specified criteria
    apply_filters(&mut combined, filter, env);

    sort_contests(&mut combined, sort_order);

    // Apply pagination
    Ok(paginate_contests(combined, page_num, page_size))
}