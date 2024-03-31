use thiserror::Error;

use super::{
    contest_bet_summary_error::ContestBetSummaryError, contest_info_error::ContestInfoError,
};

#[derive(Error, Debug, PartialEq)]
pub enum ContestActivityError {
    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    ContestBetSummaryError(#[from] ContestBetSummaryError),

    #[error(transparent)]
    ContestInfoError(#[from] ContestInfoError),
}

impl From<ContestActivityError> for cosmwasm_std::StdError {
    fn from(error: ContestActivityError) -> Self {
        cosmwasm_std::StdError::generic_err(format!(
            "Contest Activity Error: {}",
            error.to_string()
        ))
    }
}
