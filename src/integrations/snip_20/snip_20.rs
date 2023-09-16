// Reference https://github.com/scrtlabs/snip20-reference-impl/tree/master/tests/example-receiver
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, CosmosMsg, DepsMut,
    Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg, BankMsg, Coin,
};

use crate::{msg:: ExecuteMsg, state::{config, config_read}, contract::execute_from_snip_20};
use super::snip_20_msg::Snip20Msg;


pub const BLOCK_SIZE: usize = 256;


pub fn try_register(
    deps: DepsMut,
    env: Env,
    snip_20_contract_address: String,
    snip_20_contract_code_hash: String,
) -> StdResult<Response> {

    let mut conf = config(deps.storage);
    let mut state = conf.load()?;
    if !state.known_snip_20.contains(&snip_20_contract_address) {
        state.known_snip_20.push(snip_20_contract_address.clone());
    }
    conf.save(&state)?;

    let msg = to_binary(&Snip20Msg::register_receive(env.contract.code_hash))?;
    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: snip_20_contract_address,
        code_hash: snip_20_contract_code_hash,
        msg,
        funds: vec![],
    });

    Ok(Response::new().add_message(message))
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
    let state = config_read(deps.storage).load()?;
    if !state.known_snip_20.contains(&info.clone().sender.to_string()) {
        return Err(StdError::generic_err(format!(
            "XX is not a known SNIP-20 coin that this contract registered to"        )));
    }

    let mut msg: ExecuteMsg = from_binary(&msg)?;

    msg = match msg {
        ExecuteMsg::CreateContest { contest_info, contest_info_signature_hex, outcome_id, .. } => {
            ExecuteMsg::CreateContest {
                contest_info,
                contest_info_signature_hex,
                outcome_id,
                sender: Some(sender),
                amount: Some(amount),
            }
        },
        ExecuteMsg::BetContest { contest_id, outcome_id, .. } => {
            ExecuteMsg::BetContest {
                contest_id,
                outcome_id,
                sender: Some(sender),
                amount: Some(amount),
            }
        },
        _ => {
            return Err(StdError::generic_err(format!(
                " receive function only forwards CreateContest and BetContest ExecuteMsgs"            )));
        }        
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
    let state = config_read(deps.storage).load()?;
    if !state.known_snip_20.contains(&addr) {
        return Err(StdError::generic_err(format!(
            "{} is not a known SNIP-20 coin that this contract registered to",
            addr
        )));
    }
	
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