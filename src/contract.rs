use crate::contest::admin_execute_handlers::handle_set_minimum_bet;
use crate::contest::execute_handlers::{
    handle_bet_on_contest, handle_claim, handle_create_contest,
};
use crate::contest::query_handlers::{
    handle_contest_query, handle_contests_query, handle_minimum_bet_query, handle_user_bet_query,
    handle_users_bets_query,
};
use crate::error::ContractError;
use crate::integrations::oracle::oracle::query_contest_result;
use crate::integrations::snip_20::{get_supported_snip20, try_receive};
use crate::msg::{ExecuteMsg, InstantiateMsg, InvokeMsg, QueryMsg};
use crate::state::State;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use sp_secret_toolkit::master_viewing_key::MasterViewingKey;
use sp_secret_toolkit::oracle::Oracle;
use sp_secret_toolkit::snip20::Snip20;

#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State::new(
        msg.satoshis_palace,
        info.clone().sender,
        Uint128::from(1u128), // Set minimum_bet to 1
    );
    state.singleton_save(deps.storage)?;

    let snip_20 = Snip20::new(&mut deps, &env, &info, &msg.snip20, &msg.entropy);
    snip_20.singleton_save(deps.storage)?;

    MasterViewingKey::new(msg.master_viewing_key_contract).singleton_save(deps.storage)?;
    Oracle::new(msg.oracle_contract_info).singleton_save(deps.storage)?;

    Ok(Response::default()
        .add_message(snip_20.create_register_receive_msg(&env)?)
        .add_message(snip_20.create_set_view_key_msg()?))
}

#[entry_point]
pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Claim { contest_id } => handle_claim(&mut deps, env, contest_id, info.sender),
        // SNIP-20 Msgs
        ExecuteMsg::Receive {
            sender: _,
            from: _,
            amount,
            memo: _,
            msg,
        } => try_receive(deps, env, info, amount, msg),
        //Admin
        ExecuteMsg::SetMinBet { amount } => handle_set_minimum_bet(deps, info, amount),
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
        } => handle_create_contest(
            &mut deps,
            env,
            contest_info,
            contest_info_signature_hex,
            outcome_id,
            sender,
            amount,
        ),
        InvokeMsg::BetContest {
            contest_id,
            outcome_id,
            user: sender,
            amount,
        } => handle_bet_on_contest(&mut deps, &env, contest_id, outcome_id, sender, amount),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSnip20 {} => get_supported_snip20(deps),
        QueryMsg::GetContest { contest_id } => handle_contest_query(deps, contest_id),
        QueryMsg::GetContests { contest_ids } => handle_contests_query(deps, contest_ids),
        QueryMsg::GetContestResult { contest_id } => to_binary(&query_contest_result(
            &deps.querier,
            deps.storage,
            &(contest_id as u64),
        )?),
        QueryMsg::GetMinBet {} => handle_minimum_bet_query(&deps),
        _ => viewing_keys_queries(deps, msg),
    }
}

#[allow(unused)]
pub fn viewing_keys_queries(deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
    return match msg {
        QueryMsg::GetUserBet {
            user_contest,
            viewing_key,
        } => handle_user_bet_query(&deps, user_contest, viewing_key),
        QueryMsg::GetUsersBets { user, viewing_key } => {
            handle_users_bets_query(deps, user, viewing_key)
        }
        _ => Err(ContractError::QueryDoesNotRequireAuthentication.into()),
    };
}
