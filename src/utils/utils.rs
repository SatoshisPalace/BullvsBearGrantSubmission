use cosmwasm_std::{Env, MessageInfo};

use super::error::UtilError;

pub fn contract_only_call(env: Env, info: MessageInfo)-> Result<(), UtilError>{
	if env.contract.address == info.sender {
		Ok(())
	}else{
		Err(UtilError::ContractOnlyCall)
	}
}