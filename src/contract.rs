use crate::answer::ExecuteAnswer;
use crate::answer::ResponseStatus::Success;
use crate::contest::actions::{try_bet_on_contest, try_claim, try_create_contest};
use crate::contest::queries::{query_contest, query_contests, query_user_bet};
use crate::error::ContractError;
use crate::integrations::oracle::oracle::query_contest_result;
use crate::integrations::oracle::state::initialize_orace_state;
use crate::integrations::snip_20::{get_supported_snip20, try_receive};
use crate::msg::{ExecuteMsg, InstantiateMsg, InvokeMsg, QueryMsg};
use crate::state::State;
use crate::viewingkeys::viewing_keys::{try_set_key, validate_query};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use sp_secret_toolkit::snip20::Snip20;

#[entry_point]
pub fn instantiate(
    deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        satoshis_palace: msg.satoshis_palace,
        owner: deps.api.addr_canonicalize(info.sender.as_str())?,
    };
    state.singleton_save(deps.storage)?;

    let snip_20 = Snip20::new(deps, &env, &info, &msg.snip20, &msg.entropy);
    snip_20.singleton_save(deps.storage)?;

    initialize_orace_state(deps.storage, msg.oracle_contract_info);

    Ok(Response::default()
        .add_message(snip_20.create_register_receive_msg(&env)?)
        .add_message(snip_20.create_set_view_key_msg()?))
}

#[entry_point]
pub fn execute<'a>(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Claim { contest_id } => {
            match try_claim(&mut deps, &env, contest_id, info.sender) {
                Ok(response) => Ok(response
                    .clone()
                    .set_data(ExecuteAnswer::ClaimContestAnswer { status: Success })),
                Err(e) => Err(e),
            }
        }
        // SNIP-20 Msgs
        ExecuteMsg::Receive {
            sender: _,
            from: _,
            amount,
            memo: _,
            msg,
        } => try_receive(deps, env, info, amount, msg),
        //Viewing Keys
        ExecuteMsg::SetViewingKey { key, .. } => try_set_key(deps, info, key),
        //
    }
}

/**
 * This method should only ever be called from integrations::snip_20::try_receive
 */
pub fn execute_from_snip_20(
    mut deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InvokeMsg,
) -> StdResult<Response> {
    match msg {
        InvokeMsg::CreateContest {
            contest_info,
            contest_info_signature_hex,
            outcome_id,
            user: sender,
            amount,
        } => {
            try_create_contest(&mut deps, &env, &contest_info, &contest_info_signature_hex)?;
            try_bet_on_contest(
                &mut deps,
                &env,
                contest_info.id(),
                outcome_id,
                sender,
                amount,
            )?;
            Ok(
                Response::default()
                    .set_data(ExecuteAnswer::CreateContestAnswer { status: Success }),
            )
        }
        InvokeMsg::BetContest {
            contest_id,
            outcome_id,
            user: sender,
            amount,
        } => {
            try_bet_on_contest(&mut deps, &env, contest_id, outcome_id, sender, amount)?;
            Ok(Response::default().set_data(ExecuteAnswer::BetContestAnswer { status: Success }))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSnip20s {} => get_supported_snip20(deps),
        QueryMsg::GetContest { contest_id } => to_binary(&query_contest(deps, &env, contest_id)?),
        QueryMsg::GetContests { contest_ids } => {
            to_binary(&query_contests(deps, &env, contest_ids)?)
        }
        QueryMsg::GetContestResult { contest_id } => to_binary(&query_contest_result(
            &deps.querier,
            deps.storage,
            contest_id as u64,
        )?),
        _ => viewing_keys_queries(deps, msg),
    }
}

pub fn viewing_keys_queries(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    validate_query(&deps, &msg)?;

    return match msg {
        QueryMsg::GetUserBet { user_contest, .. } => {
            to_binary(&query_user_bet(&deps, user_contest)?)
        }
        _ => Err(ContractError::QueryDoesNotRequireAuthentication.into()),
    };
}
