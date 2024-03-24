use thiserror::Error;

use super::real_contest_info_error::RealContestInfoError;

#[derive(Error, Debug, PartialEq)]
pub enum ContestBetSummaryError {
    #[error("Contest Bet Summary Does Not Exist for contest with ID: {0}. Display Text: Failure to place bet. Cannot place bet on contest that does not exist.")]
    DNE(u32),

    #[error("Outcome has already been set, and connot be reset")]
    CannotResetOutcome,

    #[error("Outcome Does Not Exist. Display Text: Failure to place bet. Cannot place bet on a side that does not exist.")]
    OutcomeDNE,

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error("Failed to query the oracle for contest ID: {0}.")]
    OracleQueryFailed(u32),

    #[error(transparent)]
    ContestInfoError(#[from] RealContestInfoError),
}

impl From<ContestBetSummaryError> for cosmwasm_std::StdError {
    fn from(error: ContestBetSummaryError) -> Self {
        cosmwasm_std::StdError::generic_err(format!(
            "Contest Bet Summary Error: {}",
            error.to_string()
        ))
    }
}
