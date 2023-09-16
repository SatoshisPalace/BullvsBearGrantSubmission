use cosmwasm_std::{Binary, StdResult, to_binary, Env, Uint128, StdError, Deps};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::{msg::ExecuteMsg, integrations::snip_20::snip_20_msg::Snip20Msg};

use super::data::{contest_info::{ContestInfo, get_contest}, contest_bet_summary::{ContestBetSummary, get_contest_bet_summary}};


pub fn contest_creation_send_msg(
	env: Env,
	contest_info: ContestInfo,
	contest_info_signature_hex: String,
	outcome_id: u8,
) -> StdResult<Binary>{
	let create_contest_msg = Some(to_binary(
		&ExecuteMsg::CreateContest {
			contest_info,
			contest_info_signature_hex,
			outcome_id,
			sender: Option::None,
			amount: Option::None,
		}
	).unwrap());
	to_binary(
		&Snip20Msg::send (
			env.contract.address.into_string(),
			Some(env.contract.code_hash), 
			Uint128::one(),// Default bet is 1 this should be overwritten by front end
			create_contest_msg, 
		)
	)
}

pub fn contest_bet_send_msg(
	env: Env,
	contest_id: u32,
	outcome_id: u8,
) -> StdResult<Binary>{
	let bet_contest_msg = Some(to_binary(
		&ExecuteMsg::BetContest { 
			contest_id,
			outcome_id,
			sender: Option::None,
			amount: Option::None, 
		} 
	).unwrap());
	to_binary(
		&Snip20Msg::send(
			env.contract.address.into_string(),
			Some(env.contract.code_hash),
			Uint128::one(),// Default bet is 1 this should be overwritten by front end
			bet_contest_msg
		)
	)
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestQueryResponse {
    pub contest_info: ContestInfo,
    pub contest_bet_summary: ContestBetSummary,
}


pub fn query_contest(
    deps: Deps,
    contest_id: u32
) -> StdResult<ContestQueryResponse> {
    let contest_option: Option<ContestInfo> = get_contest(deps.storage, contest_id);
    let contest_bet_summary_option: Option<ContestBetSummary> = get_contest_bet_summary(deps.storage, contest_id);

    match (contest_option, contest_bet_summary_option) {
        (Some(contest_info), Some(contest_bet_summary)) => {
            Ok(ContestQueryResponse {
                contest_info,
                contest_bet_summary,
            })
        }
        _ => Err(StdError::generic_err(
            format!("Contest with ID:{contest_id} not found")
        )),
    }
}