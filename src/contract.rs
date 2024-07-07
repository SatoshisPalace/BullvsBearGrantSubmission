use crate::command_handlers::admin_execute_handlers::{handle_claim_fees, handle_set_minimum_bet};
use crate::command_handlers::execute_handlers::{
    handle_claim, handle_claim_multiple, handle_receive,
};
use crate::command_handlers::invoke_handlers::handle_bet_on_contest;
use crate::command_handlers::query_handlers::{
    handle_get_claimable_contests, handle_get_claimable_fees, handle_get_contest_by_id,
    handle_get_contests_by_ids, handle_get_fee_percent, handle_get_last_ten_contests,
    handle_get_minimum_bet, handle_get_snip20, handle_get_times_to_resolve_from_ids,
    handle_get_total_number_of_bets, handle_get_total_number_of_contests, handle_get_total_users,
    handle_get_total_value, handle_get_total_volume, handle_get_users_list_of_bets,
    handle_get_users_number_of_bets, handle_user_bet, handle_users_last_ten_bets,
};
use crate::data::state::{FeePercent, State};
use crate::msgs::execute::execute_msg::ExecuteMsg;
use crate::msgs::instantiate::InstantiateMsg;
use crate::msgs::invoke::invoke_msg::InvokeMsg;
use crate::msgs::query::query_msg::QueryMsg;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use sp_secret_toolkit::master_viewing_key::MasterViewingKey;
use sp_secret_toolkit::price_feed::PriceFeed;
use sp_secret_toolkit::snip20::Snip20;

#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let fee_percent = FeePercent::new(msg.fee_numerator as u128, msg.fee_denominator as u128);
    let state = State::new(
        info.sender.clone(),
        msg.interval,
        Uint128::from(1u128), // Set minimum_bet to 1
        fee_percent,
    );
    state.singleton_save(deps.storage)?;

    let snip_20 = Snip20::new(&mut deps, &env, &info, &msg.snip20, &msg.entropy);
    snip_20.singleton_save(deps.storage)?;

    MasterViewingKey::new(msg.master_viewing_key_contract).singleton_save(deps.storage)?;
    PriceFeed::new(msg.price_feed_info).singleton_save(deps.storage)?;

    Ok(Response::default()
        .add_message(snip_20.create_register_receive_msg(&env)?)
        .add_message(snip_20.create_set_view_key_msg()?))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Claim(command) => handle_claim(deps, env, info, command),
        ExecuteMsg::ClaimFees(_) => handle_claim_fees(deps, info),
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
        InvokeMsg::BetContest(command) => handle_bet_on_contest(deps, env, command, amount),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContestById(command) => handle_get_contest_by_id(deps, command),
        QueryMsg::GetContestsByIds(command) => handle_get_contests_by_ids(deps, env, command),
        QueryMsg::GetUserBet(command) => handle_user_bet(deps, command),
        QueryMsg::GetUsersLastTenBets(command) => handle_users_last_ten_bets(deps, env, command),
        QueryMsg::GetMinBet(_) => handle_get_minimum_bet(deps),
        QueryMsg::GetTotalValue(_) => handle_get_total_value(deps, env),
        QueryMsg::GetSnip20(_) => handle_get_snip20(deps),
        QueryMsg::GetClaimableFees(_) => handle_get_claimable_fees(deps),
        QueryMsg::GetClaimableContests(command) => {
            handle_get_claimable_contests(deps, env, command)
        }
        QueryMsg::GetFeePercent(_) => handle_get_fee_percent(deps),
        QueryMsg::GetTimesToResolve(command) => handle_get_times_to_resolve_from_ids(deps, command),
        QueryMsg::GetTotalNumberOfContests(_) => handle_get_total_number_of_contests(deps),
        QueryMsg::GetTotalNumberOfBets(_) => handle_get_total_number_of_bets(deps),
        QueryMsg::GetTotalVolume(_) => handle_get_total_volume(deps),
        QueryMsg::GetUsersNumberOfBets(command) => handle_get_users_number_of_bets(deps, command),
        QueryMsg::GetUsersListOfBets(command) => handle_get_users_list_of_bets(deps, env, command),
        QueryMsg::GetLastTenContests(_) => handle_get_last_ten_contests(deps, env),
        QueryMsg::GetTotalUsers(_) => handle_get_total_users(deps),
    }
}
