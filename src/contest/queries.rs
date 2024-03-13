use cosmwasm_std::{Deps, Env, StdResult};

use crate::integrations::oracle::constants::NULL_AND_VOID_CONTEST_RESULT;

use super::{
    data::{
        bets::{Bet, UserContest},
        contest_bet_summary::ContestBetSummary,
        contest_info::{ContestInfo, ContestOutcome},
    },
    error::ContestError,
    response::{ContestQueryResponse, ContestsQueryResponse, UserBetQueryResponse},
};

pub fn query_contest(deps: Deps, env: &Env, contest_id: u32) -> StdResult<ContestQueryResponse> {
    let contest_info_option: Option<ContestInfo> =
        ContestInfo::keymap_get_by_id(deps.storage, &contest_id);
    let contest_bet_summary_option: Option<ContestBetSummary> =
        ContestBetSummary::keymap_get_by_id(deps.storage, &contest_id);

    match (contest_info_option.clone(), contest_bet_summary_option) {
        (Some(contest_info), Some(mut contest_bet_summary)) => {
            let mut winner = ContestOutcome {
                id: 0,
                name: "No Winner Yet".to_string(),
            };

            if contest_info
                .assert_time_of_resolve_is_passed(env.block.time.seconds())
                .is_ok()
            {
                let contest_result = match contest_bet_summary.query_set_outcome(
                    &deps.querier,
                    deps.storage,
                    &contest_info,
                    contest_id,
                ) {
                    Ok(outcome) => outcome,
                    Err(_) => ContestOutcome {
                        id: 0,
                        name: "Contest Failed to be resolved by Oracle".to_string(),
                    },
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

pub fn query_contests(
    deps: Deps,
    env: &Env,
    contest_ids: Vec<u32>,
) -> StdResult<ContestsQueryResponse> {
    // Use filter_map to collect only the Ok results from query_contest
    let contest_query_responses: Vec<ContestQueryResponse> = contest_ids
        .into_iter()
        .filter_map(|contest_id| query_contest(deps, env, contest_id).ok())
        .collect();

    // Return the ContestsQueryResponse containing all the individual ContestQueryResponses
    Ok(ContestsQueryResponse::new(contest_query_responses))
}

pub fn query_user_bet(deps: &Deps, user_contest: UserContest) -> StdResult<UserBetQueryResponse> {
    let bet = Bet::keymap_get_by_id(deps.storage, &user_contest);
    match bet {
        Some(bet) => Ok(UserBetQueryResponse { bet }),
        None => Err(ContestError::NoBetForUserContest { user_contest }.into()),
    }
}
