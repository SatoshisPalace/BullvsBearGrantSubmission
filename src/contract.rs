use cosmwasm_std::{
     entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, State};

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
    };

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateContest {} => todo!()
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContest {} => todo!()
    }
}