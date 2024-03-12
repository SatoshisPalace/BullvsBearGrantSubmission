// Reference https://github.com/scrtlabs/snip20-reference-impl/tree/master/tests/example-receiver
use cosmwasm_std::{
    from_binary, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128, WasmMsg,
};

use super::{
    query_state::{check_known_snip_20, get_snip_20_contract},
    snip_20_msg::Snip20Msg,
    update_state::add_snip_20,
};
use crate::{contract::execute_from_snip_20, msg::InvokeMsg};

use secret_toolkit::snip20;

pub const BLOCK_SIZE: usize = 256;

pub fn try_create_register_snip20_msg(
    deps: DepsMut,
    env: Env,
    snip_20_contract_address: Addr,
    snip_20_contract_code_hash: String,
) -> StdResult<CosmosMsg> {
    add_snip_20(
        deps,
        snip_20_contract_address.clone(),
        snip_20_contract_code_hash.clone(),
    )?;

    let msg = to_binary(&Snip20Msg::register_receive(env.contract.code_hash))?;
    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: snip_20_contract_address.to_string(),
        code_hash: snip_20_contract_code_hash,
        msg,
        funds: vec![],
    });

    Ok(message)
}

#[allow(unused_assignments)]
#[allow(unused_variables)]
pub fn try_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    _from: Addr,
    amount: Uint128,
    msg: Binary,
) -> StdResult<Response> {
    check_known_snip_20(deps.storage, &info.clone().sender.to_string())?;

    let mut msg: InvokeMsg = from_binary(&msg)?;

    msg = match msg {
        InvokeMsg::CreateContest {
            contest_info,
            contest_info_signature_hex,
            outcome_id,
            user,
            ..
        } => InvokeMsg::CreateContest {
            contest_info,
            contest_info_signature_hex,
            outcome_id,
            user,
            amount: Some(amount),
        },
        InvokeMsg::BetContest {
            contest_id,
            outcome_id,
            user,
            ..
        } => InvokeMsg::BetContest {
            contest_id,
            outcome_id,
            user,
            amount: Some(amount),
        },
    };
    execute_from_snip_20(deps, env, info, msg)
}

pub fn try_redeem(
    deps: &DepsMut,
    addr: String,
    hash: String,
    to: Addr,
    amount: Uint128,
    denom: Option<String>,
) -> StdResult<Response> {
    check_known_snip_20(deps.storage, &addr)?;

    let unwrapped_denom = denom.unwrap_or("uscrt".to_string());

    let msg = to_binary(&Snip20Msg::redeem(amount, unwrapped_denom.clone()))?;
    let secret_redeem = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: addr,
        code_hash: hash,
        msg,
        funds: vec![],
    });
    let redeem = CosmosMsg::Bank(BankMsg::Send {
        // unsafe, don't use in production obviously
        amount: vec![Coin::new(amount.u128(), unwrapped_denom)],
        to_address: to.into_string(),
    });

    Ok(Response::new()
        .add_message(secret_redeem)
        .add_message(redeem))
}

pub fn send(deps: &DepsMut, recipient: String, amount: Uint128) -> StdResult<Response> {
    let snip20 = get_snip_20_contract(deps.storage, 0)?;

    let msg = snip20::send_msg(
        recipient,
        amount,
        None,
        None,
        None,
        BLOCK_SIZE,
        snip20.code_hash,
        snip20.address.to_string(),
    )?;
    Ok(Response::new().add_message(msg))
}
