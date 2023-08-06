
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CryptographyError {
	#[error("Signature Verification Failed")]
    InvalidSignature,

	#[error(transparent)]
    FromHex(#[from] hex::FromHexError),

	#[error(transparent)]
    Verification(#[from] cosmwasm_std::VerificationError),
}