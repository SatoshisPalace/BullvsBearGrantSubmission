use sp_secret_toolkit::cryptography::error::CryptographyError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RealContestInfoError {
    #[error("A contest with id: {0} already exists and cannot be reinitialized. Display Text: A contest with id: {0} already exists and cannot be reinitialized.")]
    ContestAlreadyExist(u32),
    #[error("Time of close for contest with id: {0} has passed. Display Text: Failure to place bet. Contest has closed.")]
    TimeOfClosePassed(u32),
    #[error("Time of resolve for contest with id: {contest_id} has yet to pass. Time of resolve: {time_of_resolve}, Current time: {current_time}. Display Text: Failure to claim. Contest has not been resolved.")]
    TimeOfResolveHasYetToPassed {
        contest_id: u32,
        time_of_resolve: u64,
        current_time: u64,
    },
    #[error("412: Precondition Failed. Invalid Outcome ID found in contest with ID: {contest_id}")]
    InvalidOutcomeId { contest_id: u32 },
    #[error(transparent)]
    ContestCryptagraphyError(#[from] CryptographyError),
    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error(
        "Contest with id: {0} Not Found. Display Text: Failure to claim. Contest does not exist."
    )]
    ContestNotFound(u32),
    #[error("Outcome with ID: {outcome_id}, was not found on Contest with ID: {contest_id}.")]
    OutcomeNotFound { contest_id: u32, outcome_id: u8 },
}

impl From<RealContestInfoError> for cosmwasm_std::StdError {
    fn from(error: RealContestInfoError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contest Info Error: {}", error.to_string()))
    }
}
