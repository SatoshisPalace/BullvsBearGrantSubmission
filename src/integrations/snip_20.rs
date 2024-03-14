// Reference https://github.com/scrtlabs/snip20-reference-impl/tree/master/tests/example-receiver
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use sp_secret_toolkit::{contract::contract::Contract, snip20::Snip20};

use crate::{contract::execute_from_snip_20, msg::InvokeMsg};

pub const BLOCK_SIZE: usize = 256;

pub fn try_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    msg: Binary,
) -> StdResult<Response> {
    let snip20 = Snip20::singleton_load(deps.storage)?;
    Contract::assert_address(&snip20, info.clone().sender)?;

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

pub fn get_supported_snip20(deps: Deps) -> StdResult<Binary> {
    let snip20 = Snip20::singleton_load(deps.storage)?;
    return to_binary(&snip20.get_contract_info());
}
