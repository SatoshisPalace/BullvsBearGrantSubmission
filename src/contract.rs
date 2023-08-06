use crate::contest::actions::{query_contest, try_create_contest};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, State};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
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
    };

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute<'a>(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateContest {
            contest_info,
            contest_info_signature_hex,
            users_bet: _,
        } => {
            let contest_creation = try_create_contest(
                deps,
                contest_info,
                contest_info_signature_hex,
            );
            match contest_creation {
                Ok(_) => {
                    Ok(Response::default())
                },
                Err(cryptography_error) => {
                    Err(cosmwasm_std::StdError::GenericErr { msg: cryptography_error.to_string() })
                },
            }

        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContest { contest_id } => to_binary(&query_contest(deps, contest_id)?),
    }
}
