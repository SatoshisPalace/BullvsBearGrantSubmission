use crate::cryptography;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
	#[error(transparent)]
	FromCosmWasmStd(#[from] cosmwasm_std::StdError),

	#[error(transparent)]
    FromCryptography(#[from] cryptography::error::CryptographyError),
}