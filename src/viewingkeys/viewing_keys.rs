use cosmwasm_std::{to_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

use crate::msg::QueryMsg;

use super::{
    error::ViewingKeyError,
    response::{ResponseStatus, ViewingKeyResponse},
};

pub fn try_create_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    entropy: String,
) -> StdResult<Response> {
    let key = ViewingKey::create(
        deps.storage,
        &info,
        &env,
        info.sender.as_str(),
        entropy.as_ref(),
    );

    Ok(Response::new().set_data(to_binary(&ViewingKeyResponse::CreateViewingKey { key })?))
}

pub fn try_set_key(deps: DepsMut, info: MessageInfo, key: String) -> StdResult<Response> {
    ViewingKey::set(deps.storage, info.sender.as_str(), key.as_str());
    Ok(
        Response::new().set_data(to_binary(&ViewingKeyResponse::SetViewingKey {
            status: ResponseStatus::Success,
        })?),
    )
}

pub fn validate_query(deps: &Deps, msg: &QueryMsg) -> Result<(), ViewingKeyError> {
    match msg {
        QueryMsg::GetUserBet { user_contest, key } => {
            let address = deps.api.addr_validate(user_contest.get_address_as_str())?;
            ViewingKey::check(deps.storage, address.as_str(), key.as_str())?;
            Ok(())
        }
        _ => Err(ViewingKeyError::InvalidQueryMessage),
    }
}
