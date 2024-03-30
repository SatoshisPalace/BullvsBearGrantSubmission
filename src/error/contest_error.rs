use thiserror::Error;

use super::{
    contest_bet_summary_error::ContestBetSummaryError, contest_info_error::ContestInfoError,
    state_error::StateError, user_info_error::UserInfoError,
};

#[derive(Error, Debug, PartialEq)]
pub enum ContestError {
    #[error(transparent)]
    ContestBetSummary(#[from] ContestBetSummaryError),
    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),
    #[error(transparent)]
    RealContestInfoError(#[from] ContestInfoError),
    #[error(transparent)]
    StateError(#[from] StateError),
    #[error(transparent)]
    UserInfoError(#[from] UserInfoError),
}

impl From<ContestError> for cosmwasm_std::StdError {
    fn from(contest_error: ContestError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contest Error: {}", contest_error.to_string()))
    }
}
