use cosmwasm_std::{DepsMut, StdResult, Deps, StdError};
use crate::{cryptography::cryptography::is_valid_signature, state::config_read, contract_error::ContractError};
use super::contest_info::{save_contest, get_contest, ContestInfo};

pub fn try_create_contest<'a>(
    deps: DepsMut,
    contest_info: ContestInfo,
	contest_info_signature_hex: String,
) -> Result<(), ContractError > {
	let state = config_read(deps.storage).load()?;

	let contest_info_json: String = contest_info.to_json();
	is_valid_signature(
		deps.api,
		state.satoshis_palace.as_str(),
		&contest_info_json,
		&contest_info_signature_hex,
	)?; //Populates Error if not
	save_contest(deps.storage, &contest_info);
	Ok(())
}
pub fn query_contest(
	deps: Deps,
	contest_id: u8
) -> StdResult<ContestInfo>{
	let contest_option: Option<ContestInfo> = get_contest( deps.storage, contest_id);
	match contest_option {
        Some(contest_info) => {
            return Ok(contest_info)
        }
        None => Err(StdError::generic_err(
			format!("Contest with ID:{contest_id} not found")
		)),
    }
}