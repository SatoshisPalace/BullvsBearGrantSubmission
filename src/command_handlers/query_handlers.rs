use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};
use sp_secret_toolkit::{contract::contract::Contract, snip20::Snip20};

use crate::{
    data::{
        bets::{Bet, UserContest},
        contest_bet_summary::ContestBetSummary,
        contest_info::ContestInfo,
    },
    msgs::query::commands::{
        get_claimable_contests::GetClaimableContests, get_contest_by_id::GetContestById,
        get_contests_by_ids::GetContestsByIds, get_times_to_resolve::GetTimesToResolve,
        get_user_bet::GetUserBet, get_users_last_ten_bets::GetUsersLastTenBets,
        get_users_list_of_bets::GetUsersListOfBets, get_users_number_of_bets::GetUsersNumberOfBets,
    },
    responses::query::{
        query_response::QueryResponse,
        response_types::{
            bet::UserBetResponse,
            claimable_fees::ClaimableFeesResponse,
            contest_data::ContestDataResponse,
            contest_data_list::ContestDataListResponse,
            fee_percent::FeePercentResponse,
            get_snip20::GetSnip20Response,
            minimum_bet::MinimumBetResponse,
            stats::StatsResponse,
            times_to_resolve::TimesToResolveResponse,
            total_number_of_bets::TotalNumberOfBetsResponse,
            total_number_of_contests::TotalNumberOfContestsResponse,
            total_number_of_users::TotalNumberOfUsersResponse,
            total_users_number_of_bets::TotalUsersNumberOfBetsResponse,
            total_value::TotalValueResponse,
            total_volume::TotalVolumeResponse,
            users_bets::{UserContestBetInfo, UsersBetsResponse},
        },
    },
    services::{
        bet_service::{
            get_total_bets, get_total_volume, get_user_bet, get_users_map_bets,
            get_users_number_of_bets, map_to_user_contest_bet_infos,
        },
        contest_bet_summary_service::{
            get_contest_bet_summaries_ignore_missing, get_contest_bet_summary,
            update_contest_bet_summaries_with_results,
        },
        contest_info_service::{get_contest_info, get_contest_infos_for_ids_ignore_missing},
        contests_service::{
            get_last_ten_contest_ids, get_times_to_resolve_from_contest_infos,
            get_total_number_of_contests,
        },
        integrations::master_viewing_key_service::viewing_keys::assert_valid_viewing_key,
        state_service::{get_claimable_fees, get_fee_percent, get_minimum_bet, get_snip20},
        user_info_service::{
            get_last_ten_bet_on, get_total_users, get_users_contest_bets_by_index,
        },
    },
};

pub fn handle_get_contest_by_id(deps: Deps, command: GetContestById) -> StdResult<Binary> {
    let contest_info = get_contest_info(deps.storage, &command.contest_id)?;
    let contest_bet_summary = get_contest_bet_summary(deps.storage, &command.contest_id)?;

    let response = QueryResponse::ContestData(ContestDataResponse {
        contest_info,
        contest_bet_summary,
    });
    to_binary(&response)
}

pub fn handle_get_fee_percent(deps: Deps) -> StdResult<Binary> {
    let fee_percent = get_fee_percent(deps.storage)?;
    let response = QueryResponse::FeePercent(FeePercentResponse { fee_percent });
    return to_binary(&response);
}

pub fn handle_get_minimum_bet(deps: Deps) -> StdResult<Binary> {
    let minimum_bet = get_minimum_bet(deps.storage)?;
    let response = QueryResponse::MinimumBet(MinimumBetResponse { minimum_bet });
    return to_binary(&response);
}

pub fn handle_get_claimable_fees(deps: Deps) -> StdResult<Binary> {
    let claimable_fees = get_claimable_fees(deps.storage)?;
    let response = QueryResponse::ClaimableFees(ClaimableFeesResponse { claimable_fees });
    return to_binary(&response);
}

pub fn handle_get_total_value(deps: Deps, env: Env) -> StdResult<Binary> {
    let storage = deps.storage;

    let snip20 = Snip20::singleton_load(storage)?;

    let balance = snip20.query_contract_balance(&deps.querier, &env).unwrap();

    let total_value = balance.amount;

    let response = QueryResponse::TotalValue(TotalValueResponse { total_value });
    return to_binary(&response);
}

pub fn handle_get_users_number_of_bets(
    deps: Deps,
    command: GetUsersNumberOfBets,
) -> StdResult<Binary> {
    assert_valid_viewing_key(
        deps.storage,
        &deps.querier,
        &command.user,
        &command.viewing_key,
    )?;

    let total_users_number_of_bets = get_users_number_of_bets(deps.storage, &command.user);

    let response = QueryResponse::TotalUsersNumberOfBets(TotalUsersNumberOfBetsResponse {
        total_users_number_of_bets,
    });
    return to_binary(&response);
}

pub fn handle_get_users_list_of_bets(
    deps: Deps,
    env: Env,
    command: GetUsersListOfBets,
) -> StdResult<Binary> {
    assert_valid_viewing_key(
        deps.storage,
        &deps.querier,
        &command.user,
        &command.viewing_key,
    )?;

    let contest_ids =
        get_users_contest_bets_by_index(deps.storage, &command.contest_ids, &command.user)?;

    let contest_infos = get_contest_infos_for_ids_ignore_missing(deps.storage, &contest_ids);
    let mut bets: Vec<Bet> = vec![];
    let mut contest_bet_summaries =
        get_contest_bet_summaries_ignore_missing(deps.storage, &contest_ids);
    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        &env,
        &contest_infos,
        &mut contest_bet_summaries,
    );

    for id in contest_ids {
        bets.push(get_user_bet(
            deps.storage,
            UserContest::new(command.user.clone(), id),
        )?);
    }

    // Ensure all vectors are the same length
    assert_eq!(contest_infos.len(), contest_bet_summaries.len());
    assert_eq!(contest_bet_summaries.len(), bets.len());

    let collected_results: Vec<(ContestInfo, ContestBetSummary, Bet)> = contest_infos
        .into_iter()
        .zip(contest_bet_summaries.into_iter())
        .zip(bets.into_iter())
        .map(|((contest_info, contest_bet_summary), bet)| (contest_info, contest_bet_summary, bet))
        .collect();

    let contests_bets: Vec<UserContestBetInfo> = map_to_user_contest_bet_infos(collected_results);

    let response = QueryResponse::UsersBets(UsersBetsResponse { contests_bets });
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

pub fn handle_users_last_ten_bets(
    deps: Deps,
    env: Env,
    command: GetUsersLastTenBets,
) -> StdResult<Binary> {
    assert_valid_viewing_key(
        deps.storage,
        &deps.querier,
        &command.user,
        &command.viewing_key,
    )?;

    let last_10 = get_last_ten_bet_on(deps.storage, &command.user)?;

    let contest_infos = get_contest_infos_for_ids_ignore_missing(deps.storage, &last_10);
    let mut bets: Vec<Bet> = vec![];
    let mut contest_bet_summaries =
        get_contest_bet_summaries_ignore_missing(deps.storage, &last_10);
    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        &env,
        &contest_infos,
        &mut contest_bet_summaries,
    );

    for id in last_10 {
        bets.push(get_user_bet(
            deps.storage,
            UserContest::new(command.user.clone(), id),
        )?);
    }

    // Ensure all vectors are the same length
    assert_eq!(contest_infos.len(), contest_bet_summaries.len());
    assert_eq!(contest_bet_summaries.len(), bets.len());

    let collected_results: Vec<(ContestInfo, ContestBetSummary, Bet)> = contest_infos
        .into_iter()
        .zip(contest_bet_summaries.into_iter())
        .zip(bets.into_iter())
        .map(|((contest_info, contest_bet_summary), bet)| (contest_info, contest_bet_summary, bet))
        .collect();

    let contests_bets: Vec<UserContestBetInfo> = map_to_user_contest_bet_infos(collected_results);

    let response = QueryResponse::UsersBets(UsersBetsResponse { contests_bets });
    return to_binary(&response);
}

pub fn handle_get_snip20(deps: Deps) -> StdResult<Binary> {
    let snip20 = get_snip20(deps.storage)?;
    let response = QueryResponse::Snip20(GetSnip20Response {
        snip20: Contract::get_contract_info(&snip20),
    });
    return to_binary(&response);
}

pub fn handle_get_claimable_contests(
    deps: Deps,
    env: Env,
    command: GetClaimableContests,
) -> StdResult<Binary> {
    let GetClaimableContests { user, viewing_key } = command;

    assert_valid_viewing_key(deps.storage, &deps.querier, &user, &viewing_key)?;

    // Filter contests, bet summaries, and bets based on the provided filters
    let filtered_results = get_users_map_bets(deps, env, user)?;

    // Construct UserContestBetInfo
    let contests_bets: Vec<UserContestBetInfo> = map_to_user_contest_bet_infos(filtered_results);

    let response = QueryResponse::UsersBets(UsersBetsResponse { contests_bets });

    to_binary(&response)
}

pub fn handle_get_contests_by_ids(
    deps: Deps,
    env: Env,
    command: GetContestsByIds,
) -> StdResult<Binary> {
    let contest_infos =
        get_contest_infos_for_ids_ignore_missing(deps.storage, &command.contest_ids);
    let mut contest_bet_summaries =
        get_contest_bet_summaries_ignore_missing(deps.storage, &command.contest_ids);

    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        &env,
        &contest_infos,
        &mut contest_bet_summaries,
    );

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
    return to_binary(&response);
}

pub fn handle_get_times_to_resolve_from_ids(
    deps: Deps,
    command: GetTimesToResolve,
) -> StdResult<Binary> {
    let contest_infos =
        get_contest_infos_for_ids_ignore_missing(deps.storage, &command.contest_ids);

    let times = get_times_to_resolve_from_contest_infos(&deps, contest_infos);
    let response = QueryResponse::TimesToResolve(TimesToResolveResponse { times });
    return to_binary(&response);
}

pub fn handle_get_last_ten_contests(deps: Deps, env: Env) -> StdResult<Binary> {
    let contest_ids = get_last_ten_contest_ids(deps.storage);

    let contest_infos = get_contest_infos_for_ids_ignore_missing(deps.storage, &contest_ids);
    let mut contest_bet_summaries =
        get_contest_bet_summaries_ignore_missing(deps.storage, &contest_ids);

    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        &env,
        &contest_infos,
        &mut contest_bet_summaries,
    );

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
    return to_binary(&response);
}

pub fn handle_get_stats(deps: Deps) -> StdResult<Binary> {
    let contests = get_total_number_of_contests(deps.storage);

    let bets = get_total_bets(deps.storage);

    let users = get_total_users(deps.storage);

    let volume = get_total_volume(deps.storage);

    let response = QueryResponse::Stats(StatsResponse {
        contests,
        bets,
        users,
        volume,
    });

    return to_binary(&response);
}

pub fn handle_get_total_number_of_contests(deps: Deps) -> StdResult<Binary> {
    let total_number_of_contests = get_total_number_of_contests(deps.storage);
    let response = QueryResponse::TotalNumberOfContests(TotalNumberOfContestsResponse {
        total_number_of_contests: total_number_of_contests.try_into().unwrap(),
    });
    return to_binary(&response);
}

pub fn handle_get_total_number_of_bets(deps: Deps) -> StdResult<Binary> {
    let total_number_of_bets = get_total_bets(deps.storage);
    let response = QueryResponse::TotalNumberOfBets(TotalNumberOfBetsResponse {
        total_number_of_bets,
    });
    return to_binary(&response);
}

pub fn handle_get_total_users(deps: Deps) -> StdResult<Binary> {
    let total_number_of_users = get_total_users(deps.storage);
    let response = QueryResponse::TotalNumberOfUsers(TotalNumberOfUsersResponse {
        total_number_of_users,
    });
    return to_binary(&response);
}

pub fn handle_get_total_volume(deps: Deps) -> StdResult<Binary> {
    let total_volume = get_total_volume(deps.storage);
    let response = QueryResponse::TotalVolume(TotalVolumeResponse { total_volume });
    return to_binary(&response);
}
