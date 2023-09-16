
use thiserror::Error;

use crate::cryptography::error::CryptographyError;

#[derive(Error, Debug, PartialEq)]
pub enum ContestError {
	#[error("Contest Does Not Exist")]
    ContestDNE,

	#[error("A contest with id: {0} already exists and cannot be reinitialized")]
    ContestAlreadyExist(u32),

	#[error("Outcome Does Not Exist")]
	OutcomeDNE,

	#[error("Bet on contest is insufficient")]
	BetInsufficient,

	#[error("Bet made without amount")]
	BetHasNoAmount,

	#[error("Bet made without a sender")]
	BetHasNoSender,

	#[error("Message passed from snip-20 was not a contest bet")]
	MessageNotContestBet,

	#[error(transparent)]
	StandardError(#[from] cosmwasm_std::StdError),

	#[error(transparent)]
	ContestCryptagraphyError(#[from] CryptographyError),
}

impl From<ContestError> for cosmwasm_std::StdError {
    fn from(contest_error: ContestError) -> Self {
        return cosmwasm_std::StdError::GenericErr{ msg: contest_error.to_string() };
    }
}