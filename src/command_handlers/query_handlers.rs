use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};
use sp_secret_toolkit::{contract::contract::Contract, snip20::Snip20};

use crate::{
    data::{
        bets::{Bet, UserContest},
        contest_bet_summary::ContestBetSummary,
        contest_info::ContestInfo,
    },
    msgs::query::commands::{
        get_contest_by_id::GetContestById,
        get_contests::GetContests,
        get_contests_by_ids::GetContestsByIds,
        get_user_bet::GetUserBet,
        get_users_bets::{GetUsersBets, UsersBetsQueryFilters},
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
            total_value::TotalValueResponse,
            users_bets::{UserContestBetInfo, UsersBetsResponse},
        },
    },
    services::{
        bet_service::{get_bets_for_user_and_contests, get_user_bet},
        contest_bet_summary_service::{
            get_contest_bet_summaries, get_contest_bet_summaries_ignore_missing,
            get_contest_bet_summary, update_contest_bet_summaries_with_results,
        },
        contest_info_service::{
            get_contest_info, get_contest_infos_for_ids, get_contest_infos_for_ids_ignore_missing,
        },
        contests_service::get_contests,
        integrations::master_viewing_key_service::viewing_keys::assert_valid_viewing_key,
        state_service::{get_claimable_fees, get_fee_percent, get_minimum_bet, get_snip20},
        user_info_service::get_contests_for_user,
    },
};

pub fn handle_users_bets_query(deps: Deps, env: Env, command: GetUsersBets) -> StdResult<Binary> {
    let GetUsersBets {
        user,
        viewing_key,
        filters,
    } = command;

    assert_valid_viewing_key(deps.storage, &deps.querier, &user, &viewing_key)?;

    let users_contest_ids = get_contests_for_user(deps.storage, &user)?;
    let users_contest_infos = get_contest_infos_for_ids(deps.storage, &users_contest_ids)?;
    let mut users_contest_bet_summaries =
        get_contest_bet_summaries(deps.storage, &users_contest_ids)?;
    update_contest_bet_summaries_with_results(
        deps.storage,
        &deps.querier,
        &env,
        &users_contest_infos,
        &mut users_contest_bet_summaries,
    );
    let users_bets = get_bets_for_user_and_contests(deps.storage, &user, &users_contest_ids)?;

    // Filter contests, bet summaries, and bets based on the provided filters
    let filtered_results = filter_contests(
        &users_contest_infos,
        &users_contest_bet_summaries,
        &users_bets,
        &filters,
    );

    // Construct UserContestBetInfo or a similar structure for each filtered result
    let contests_bets: Vec<UserContestBetInfo> = filtered_results
        .into_iter()
        .map(
            |(contest_info, contest_bet_summary, user_bet)| UserContestBetInfo {
                contest_info,
                contest_bet_summary,
                user_bet,
            },
        )
        .collect();

    let response = QueryResponse::UsersBets(UsersBetsResponse { contests_bets });

    to_binary(&response)
}

pub fn handle_get_contest_by_id(deps: Deps, command: GetContestById) -> StdResult<Binary> {
    let contest_info = get_contest_info(deps.storage, &command.contest_id)?;
    let contest_bet_summary = get_contest_bet_summary(deps.storage, &command.contest_id)?;

    let response = QueryResponse::ContestData(ContestDataResponse {
        contest_info,
        contest_bet_summary,
    });
    to_binary(&response)
}

pub fn handle_get_contests_by_ids(deps: Deps, command: GetContestsByIds) -> StdResult<Binary> {
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
    return to_binary(&response);
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

pub fn filter_contests(
    contest_infos: &Vec<ContestInfo>,
    contest_bet_summaries: &Vec<ContestBetSummary>,
    bets: &Vec<Bet>,
    filters: &Option<Vec<UsersBetsQueryFilters>>,
) -> Vec<(ContestInfo, ContestBetSummary, Bet)> {
    let is_claimable_filter_active =
        matches!(filters, Some(filters) if filters.contains(&UsersBetsQueryFilters::Claimable));

    contest_infos
        .iter()
        .zip(contest_bet_summaries.iter())
        .zip(bets.iter())
        .filter_map(
            |((contest_info, contest_bet_summary), bet)| match is_claimable_filter_active {
                true => {
                    if bet.has_been_paid() {
                        None
                    } else {
                        match contest_bet_summary.get_outcome() {
                            Some(outcome) if outcome.get_id() == bet.get_outcome_id() => Some((
                                (*contest_info).clone(),
                                (*contest_bet_summary).clone(),
                                (*bet).clone(),
                            )),
                            _ => None,
                        }
                    }
                }
                _ => Some((
                    (*contest_info).clone(),
                    (*contest_bet_summary).clone(),
                    (*bet).clone(),
                )),
            },
        )
        .collect()
}

pub fn handle_get_contests(deps: Deps, env: Env, command: GetContests) -> StdResult<Binary> {
    let GetContests {
        page_num,
        page_size,
        sort_order,
        filter,
    } = command;

    // Use the `?` operator to simplify error handling
    let contest_pairs = get_contests(&deps, &env, page_num, page_size, sort_order, filter)?;

    // Transform the data into the response format
    let contests: Vec<ContestDataResponse> = contest_pairs
        .into_iter()
        .map(|(contest_info, contest_bet_summary)| ContestDataResponse {
            contest_info,
            contest_bet_summary,
        })
        .collect();
    let response = QueryResponse::ContestDataList(ContestDataListResponse { contests });

    // Serialize the response into binary format and return
    return to_binary(&response);
}
