use thiserror::Error;


#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
	#[error("Unsupported message variant for execute. try sending via snip-20?")]
	UnsupportedExecuteMessage,

	#[error("Execute Message received from snip 20 not supported")]
	UnsupportedSnip20ExecuteMsg,

	#[error("The Received Query does not require authentication")]
	QueryDoesNotRequireAuthentication
}

impl From<ContractError> for cosmwasm_std::StdError {
    fn from(contract_error: ContractError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contract Error: {}", contract_error.to_string()))
    }
}
