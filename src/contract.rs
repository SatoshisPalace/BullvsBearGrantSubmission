use crate::command_handlers::admin_execute_handlers::handle_set_minimum_bet;
use crate::command_handlers::execute_handlers::{
    handle_claim, handle_claim_multiple, handle_receive,
};
use crate::command_handlers::invoke_handlers::{handle_bet_on_contest, handle_create_contest};
use crate::command_handlers::query_handlers::{
    handle_get_active_contests, handle_get_contest, handle_get_contests, handle_get_minimum_bet,
    handle_get_snip20, handle_user_bet, handle_users_bets_query,
};
use crate::data::contest_activity::ContestActivity;
use crate::data::state::State;
use crate::msgs::execute::execute_msg::ExecuteMsg;
use crate::msgs::instantiate::InstantiateMsg;
use crate::msgs::invoke::invoke_msg::InvokeMsg;
use crate::msgs::query::query_msg::QueryMsg;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
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

    let contest_activity = ContestActivity::new();
    contest_activity.singleton_save(deps.storage)?;

    Ok(Response::default()
        .add_message(snip_20.create_register_receive_msg(&env)?)
        .add_message(snip_20.create_set_view_key_msg()?))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Claim(command) => handle_claim(deps, env, info, command),
        ExecuteMsg::ClaimMultiple(command) => handle_claim_multiple(deps, env, info, command),
        ExecuteMsg::SetMinimumBet(command) => handle_set_minimum_bet(deps, info, command),
        ExecuteMsg::Receive(command) => handle_receive(deps, env, info, command),
    }
}

/**
 * This method should only ever be called from integrations::snip_20::try_receive
 */
pub fn invoke(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InvokeMsg,
    amount: Uint128,
) -> StdResult<Response> {
    match msg {
        InvokeMsg::CreateContest(command) => handle_create_contest(deps, env, command, amount),
        InvokeMsg::BetContest(command) => handle_bet_on_contest(deps, env, command, amount),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContest(command) => handle_get_contest(deps, command),
        QueryMsg::GetContests(command) => handle_get_contests(deps, command),
        QueryMsg::GetActiveContests(command) => handle_get_active_contests(deps, env, command),
        QueryMsg::GetUserBet(command) => handle_user_bet(deps, command),
        QueryMsg::GetUsersBets(command) => handle_users_bets_query(deps, env, command),
        QueryMsg::GetMinBet(_) => handle_get_minimum_bet(deps),
        QueryMsg::GetSnip20(_) => handle_get_snip20(deps),
    }
}
