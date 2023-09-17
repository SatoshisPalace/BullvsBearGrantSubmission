use crate::contest::actions::{try_bet_on_contest, try_claim, try_create_contest};
use crate::contest::queries::{
    contest_bet_send_msg, contest_creation_send_msg, query_contest, query_user_bet,
};
use crate::integrations::snip_20::query_state::get_registered_snip_20s;
use crate::integrations::snip_20::snip_20::{try_receive, try_redeem, try_register};
use crate::integrations::snip_20::update_state::initialize_snip_20_state;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, State};
use crate::viewingkeys::viewing_keys::{try_create_key, try_set_key, validate_query};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        satoshis_palace: msg.satoshis_palace,
        oracle_contract: msg.oracle_contract,
        owner: deps.api.addr_canonicalize(info.sender.as_str())?,
    };
    config(deps.storage).save(&state)?;

    initialize_snip_20_state(deps.storage);

    Ok(Response::default())
}

#[entry_point]
pub fn execute<'a>(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Claim { contest_id } => try_claim(&mut deps, contest_id, info.sender),
        // SNIP-20 Msgs
        ExecuteMsg::Register { reg_addr, reg_hash } => try_register(deps, env, reg_addr, reg_hash),
        ExecuteMsg::Receive {
            sender,
            from,
            amount,
            memo: _,
            msg,
        } => try_receive(deps, env, info, sender, from, amount, msg),
        ExecuteMsg::Redeem {
            addr,
            hash,
            to,
            amount,
            denom,
        } => try_redeem(&deps, addr, hash, to, amount, denom),
        //Viewing Keys
        ExecuteMsg::CreateViewingKey { entropy, .. } => try_create_key(deps, env, info, entropy),
        ExecuteMsg::SetViewingKey { key, .. } => try_set_key(deps, info, key),
        //
        _ => Err(StdError::generic_err(
            "Unsupported message variant for execute. try sending via snip-20?",
        )),
    }
}

/**
 * This method should only ever be called from integrations::snip_20::try_receive
 */
pub fn execute_from_snip_20(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateContest {
            contest_info,
            contest_info_signature_hex,
            outcome_id,
            sender,
            amount,
        } => {
            try_create_contest(&mut deps, &contest_info, &contest_info_signature_hex)?;
            try_bet_on_contest(&mut deps, contest_info.id(), outcome_id, sender, amount)?;
            Ok(Response::default())
        }
        ExecuteMsg::BetContest {
            contest_id,
            outcome_id,
            sender,
            amount,
        } => {
            try_bet_on_contest(&mut deps, contest_id, outcome_id, sender, amount)?;
            Ok(Response::default())
        }
        _ => Err(StdError::generic_err(
            "Unsupported message variant for execute_from_snip_20",
        )),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSnip20s {} => get_registered_snip_20s(deps.storage),
        QueryMsg::GetContest { contest_id } => to_binary(&query_contest(deps, contest_id)?),
        QueryMsg::GetContestCreationMsgBinary {
            contest_info,
            contest_info_signature_hex,
            outcome_id,
        } => contest_creation_send_msg(env, contest_info, contest_info_signature_hex, outcome_id),
        QueryMsg::GetBetContestMsgBinary {
            contest_id,
            outcome_id,
        } => contest_bet_send_msg(env, contest_id, outcome_id),

        _ => viewing_keys_queries(deps, msg),
    }
}

pub fn viewing_keys_queries(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    validate_query(&deps, &msg)?;

    return match msg {
        QueryMsg::GetUserBet { user_contest, .. } => {
            to_binary(&query_user_bet(&deps, user_contest)?)
        }
        _ => panic!("This query type does not require authentication"),
    };
}
