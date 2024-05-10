use sp_secret_toolkit::macros::identifiable::Identifiable;
use thiserror::Error;

use crate::data::contest_info::ContestInfo;

#[derive(Error, Debug, PartialEq)]
pub enum ContestInfoError {
    #[error("A contest with id: {0} already exists and cannot be reinitialized. Display Text: A contest with id: {0} already exists and cannot be reinitialized.")]
    ContestAlreadyExist(String),

    #[error("Time of close for contest with id: {0} has passed. Display Text: Failure to place bet. Contest has closed.")]
    TimeOfClosePassed(String),

    #[error("Time of resolve for contest with id: {0} has passed. Display Text: Failure to place bet. Contest has passed time of resolve.")]
    TimeOfResolvePassed(String),

    #[error("Time of resolve for contest with id: {contest_id} has yet to pass. Time of resolve: {time_of_resolve}, Current time: {current_time}. Display Text: Failure to claim. Contest has not been resolved.")]
    TimeOfResolveHasYetToPassed {
        contest_id: <ContestInfo as Identifiable>::ID,
        time_of_resolve: u64,
        current_time: u64,
    },

    #[error("412: Precondition Failed. Invalid Outcome ID found in contest with ID: {contest_id}")]
    InvalidOutcomeId { contest_id: <ContestInfo as Identifiable>::ID },

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error(
        "Contest with id: {0} Not Found. Display Text: Failure to claim. Contest does not exist."
    )]
    ContestNotFound(String),

    #[error("Outcome with ID: {outcome_id}, was not found on Contest with ID: {contest_id}.")]
    OutcomeNotFound { contest_id: <ContestInfo as Identifiable>::ID, outcome_id: u8 },

    #[error("Contest Does Not Exist. Display Text: Failure to place bet. Cannot place bet on contest that does not exist.")]
    ContestDNE,

    #[error("Outcome Does Not Exist. Display Text: Failure to place bet. Cannot place bet on a side that does not exist.")]
    OutcomeDNE,
}

impl From<ContestInfoError> for cosmwasm_std::StdError {
    fn from(error: ContestInfoError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contest Info Error: {}", error.to_string()))
    }
}
