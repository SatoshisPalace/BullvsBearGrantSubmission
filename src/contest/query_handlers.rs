use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult};

use crate::integrations::viewing_keys::viewing_keys::assert_valid_viewing_key;

use super::{
    data::bets::UserContest,
    responses::query::{
        bet_response::UserBetQueryResponse,
        contest_response::{ContestInfoAndSummaryQueryResponse, ContestsQueryResponse},
        state_response::MinimumBetResponse,
        users_bets_response::{UserBetsResponse, UserContestBetInfo},
    },
    services::{
        bet_service::{get_bets_for_user_and_contests, get_user_bet},
        contest_bet_summary_service::{
            get_contest_bet_summaries_ignore_missing, get_contest_bet_summary,
        },
        contest_info_service::{
            get_contest_info, get_contest_infos_for_ids, get_contest_infos_for_ids_ignore_missing,
        },
        state_service::get_minimum_bet,
        user_info_service::get_contests_for_user,
    },
};

pub fn handle_users_bets_query(deps: Deps, user: Addr, viewing_key: String) -> StdResult<Binary> {
    assert_valid_viewing_key(deps.storage, &deps.querier, &user, &viewing_key)?;

    let users_contest_ids = get_contests_for_user(deps.storage, &user)?;
    let users_contest_infos = get_contest_infos_for_ids(deps.storage, &users_contest_ids)?;
    let users_bets = get_bets_for_user_and_contests(deps.storage, &user, &users_contest_ids)?;

    // Combine contest infos and bets into UserContestBetInfo structs
    let contests_bets: Vec<UserContestBetInfo> = users_contest_infos
        .into_iter()
        .zip(users_bets.into_iter())
        .map(|(contest_info, user_bet)| UserContestBetInfo {
            contest_info,
            user_bet,
        })
        .collect();

    let response = UserBetsResponse { contests_bets };

    to_binary(&response)
}

pub fn handle_contest_query(deps: Deps, contest_id: u32) -> StdResult<Binary> {
    let contest_info = get_contest_info(deps.storage, &contest_id)?;
    let contest_bet_summary = get_contest_bet_summary(deps.storage, &contest_id)?;
    let response = ContestInfoAndSummaryQueryResponse {
        contest_info,
        contest_bet_summary,
    };
    to_binary(&response)
}

pub fn handle_contests_query(deps: Deps, contest_ids: Vec<u32>) -> StdResult<Binary> {
    let contest_infos = get_contest_infos_for_ids_ignore_missing(deps.storage, &contest_ids);
    let contest_bet_summaries =
        get_contest_bet_summaries_ignore_missing(deps.storage, &contest_ids);

    let contest_infos_and_summaries: Vec<ContestInfoAndSummaryQueryResponse> = contest_infos
        .into_iter()
        .zip(contest_bet_summaries.into_iter())
        .map(
            |(contest_info, contest_bet_summary)| ContestInfoAndSummaryQueryResponse {
                contest_info,
                contest_bet_summary,
            },
        )
        .collect();

    let response = ContestsQueryResponse {
        contests: contest_infos_and_summaries,
    };

    to_binary(&response)
}

pub fn handle_minimum_bet_query(deps: &Deps) -> StdResult<Binary> {
    let response: MinimumBetResponse = MinimumBetResponse {
        minimum_bet: get_minimum_bet(deps.storage)?,
    };
    return to_binary(&response);
}

pub fn handle_user_bet_query(
    deps: &Deps,
    user_contest: UserContest,
    viewing_key: String,
) -> StdResult<Binary> {
    assert_valid_viewing_key(
        deps.storage,
        &deps.querier,
        &user_contest.get_address(),
        &viewing_key,
    )?;

    let bet = get_user_bet(deps.storage, user_contest)?;
    let response: UserBetQueryResponse = UserBetQueryResponse { bet };
    return to_binary(&response);
}
