use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use sp_secret_toolkit::contract::contract::Contract;

use crate::{
    data::bets::UserContest,
    msgs::query::commands::{
        get_contest::GetContest, get_contests::GetContests, get_user_bet::GetUserBet,
        get_users_bets::GetUsersBets,
    },
    responses::query::{
        query_response::QueryResponse,
        response_types::{
            bet::UserBetResponse,
            contest_data::ContestDataResponse,
            contest_data_list::ContestDataListResponse,
            get_snip20::GetSnip20Response,
            minimum_bet::MinimumBetResponse,
            users_bets::{UserBetsResponse, UserContestBetInfo},
        },
    },
    services::{
        bet_service::{get_bets_for_user_and_contests, get_user_bet},
        contest_bet_summary_service::{
            get_contest_bet_summaries_ignore_missing, get_contest_bet_summary,
        },
        contest_info_service::{
            get_contest_info, get_contest_infos_for_ids, get_contest_infos_for_ids_ignore_missing,
        },
        integrations::master_viewing_key_service::viewing_keys::assert_valid_viewing_key,
        state_service::{get_minimum_bet, get_snip20},
        user_info_service::get_contests_for_user,
    },
};

pub fn handle_users_bets_query(deps: Deps, command: GetUsersBets) -> StdResult<Binary> {
    assert_valid_viewing_key(
        deps.storage,
        &deps.querier,
        &command.user,
        &command.viewing_key,
    )?;

    let users_contest_ids = get_contests_for_user(deps.storage, &command.user)?;
    let users_contest_infos = get_contest_infos_for_ids(deps.storage, &users_contest_ids)?;
    let users_bets =
        get_bets_for_user_and_contests(deps.storage, &command.user, &users_contest_ids)?;

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

pub fn handle_get_contest(deps: Deps, command: GetContest) -> StdResult<Binary> {
    let contest_info = get_contest_info(deps.storage, &command.contest_id)?;
    let contest_bet_summary = get_contest_bet_summary(deps.storage, &command.contest_id)?;

    let response = QueryResponse::ContestData(ContestDataResponse {
        contest_info,
        contest_bet_summary,
    });
    to_binary(&response)
}

pub fn handle_get_contests(deps: Deps, command: GetContests) -> StdResult<Binary> {
    let contest_infos =
        get_contest_infos_for_ids_ignore_missing(deps.storage, &command.contest_ids);
    let contest_bet_summaries =
        get_contest_bet_summaries_ignore_missing(deps.storage, &command.contest_ids);

    let contest_infos_and_summaries: Vec<ContestDataResponse> = contest_infos
        .into_iter()
        .zip(contest_bet_summaries.into_iter())
        .map(|(contest_info, contest_bet_summary)| ContestDataResponse {
            contest_info,
            contest_bet_summary,
        })
        .collect();

    let response = QueryResponse::ContestDataList(ContestDataListResponse {
        contests: contest_infos_and_summaries,
    });
    to_binary(&response)
}

pub fn handle_get_minimum_bet(deps: Deps) -> StdResult<Binary> {
    let minimum_bet = get_minimum_bet(deps.storage)?;
    let response = QueryResponse::MinimumBet(MinimumBetResponse { minimum_bet });
    return to_binary(&response);
}

pub fn handle_user_bet(deps: Deps, command: GetUserBet) -> StdResult<Binary> {
    assert_valid_viewing_key(
        deps.storage,
        &deps.querier,
        &command.user,
        &command.viewing_key,
    )?;

    let user_contest = UserContest::new(command.user, command.contest_id);
    let bet = get_user_bet(deps.storage, user_contest)?;
    let response = QueryResponse::UserBet(UserBetResponse { bet });
    return to_binary(&response);
}

pub fn handle_get_snip20(deps: Deps) -> StdResult<Binary> {
    let snip20 = get_snip20(deps.storage)?;
    let response = QueryResponse::Snip20(GetSnip20Response {
        snip20: Contract::get_contract_info(&snip20),
    });
    return to_binary(&response);
}
