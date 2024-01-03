use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult, Uint128};

use crate::{integrations::{snip_20::snip_20_msg::Snip20Msg, oracle::constants::NULL_AND_VOID_CONTEST_RESULT}, msg::ExecuteMsg};

use super::{
    data::{
        bets::{get_bet, UserContest},
        contest_bet_summary::{get_contest_bet_summary, ContestBetSummary},
        contest_info::{get_contest, ContestInfo, ContestOutcome},
    },
    error::ContestError,
    response::{ContestQueryResponse, UserBetQueryResponse, ContestsQueryResponse},
};

pub fn contest_creation_send_msg(
    env: Env,
    contest_info: ContestInfo,
    contest_info_signature_hex: String,
    outcome_id: u8,
) -> StdResult<Binary> {
    let create_contest_msg = Some(
        to_binary(&ExecuteMsg::CreateContest {
            contest_info,
            contest_info_signature_hex,
            outcome_id,
            sender: Option::None,
            amount: Option::None,
        })
        .unwrap(),
    );
    to_binary(&Snip20Msg::send(
        env.contract.address.into_string(),
        Some(env.contract.code_hash),
        Uint128::one(), // Default bet is 1 this should be overwritten by front end
        create_contest_msg,
    ))
}

pub fn contest_bet_send_msg(env: Env, contest_id: u32, outcome_id: u8) -> StdResult<Binary> {
    let bet_contest_msg = Some(
        to_binary(&ExecuteMsg::BetContest {
            contest_id,
            outcome_id,
            sender: Option::None,
            amount: Option::None,
        })
        .unwrap(),
    );
    to_binary(&Snip20Msg::send(
        env.contract.address.into_string(),
        Some(env.contract.code_hash),
        Uint128::one(), // Default bet is 1 this should be overwritten by front end
        bet_contest_msg,
    ))
}

pub fn query_contest(deps: Deps, env: &Env, contest_id: u32) -> StdResult<ContestQueryResponse> {
    let contest_info_option: Option<ContestInfo> = get_contest(deps.storage, contest_id);
    let contest_bet_summary_option: Option<ContestBetSummary> = get_contest_bet_summary(deps.storage, contest_id);

    match (contest_info_option.clone(), contest_bet_summary_option) {
        (Some(contest_info), Some(mut contest_bet_summary)) => {
            let mut winner = ContestOutcome { id: 0, name: "No Winner Yet".to_string() };

            if contest_info.assert_time_of_resolve_is_passed(env.block.time.seconds()).is_ok() {
                let contest_result = match contest_bet_summary.get_outcome(&deps.querier, deps.storage, &contest_info, contest_id) {
                    Ok(outcome) => outcome,
                    Err(_) => ContestOutcome { id: 0, name: "Contest Failed to be resolved by Oracle".to_string() },
                };

                if contest_result.id != NULL_AND_VOID_CONTEST_RESULT {
                    winner = contest_result;
                }
            }

            Ok(ContestQueryResponse {
                contest_info,
                contest_bet_summary,
                contest_winner: winner,
            })
        }
        _ => Err(ContestError::ContestNotFound(contest_id).into()),
    }
}





pub fn query_contests(deps: Deps,  env: &Env, contest_ids: Vec<u32>) -> StdResult<ContestsQueryResponse> {
    // Use filter_map to collect only the Ok results from query_contest
    let contest_query_responses: Vec<ContestQueryResponse> = contest_ids
        .into_iter()
        .filter_map(|contest_id| query_contest(deps, env,  contest_id).ok())
        .collect();

    // Return the ContestsQueryResponse containing all the individual ContestQueryResponses
    Ok(ContestsQueryResponse::new(contest_query_responses))
}

pub fn query_user_bet(deps: &Deps, user_contest: UserContest) -> StdResult<UserBetQueryResponse> {
    let bet = get_bet(deps.storage, &user_contest);
    match bet {
        Some(bet) => Ok(UserBetQueryResponse { bet }),
        None => Err(ContestError::NoBetForUserContest { user_contest }.into()),
    }
}
