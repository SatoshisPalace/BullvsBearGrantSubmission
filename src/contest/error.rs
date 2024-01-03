use thiserror::Error;

use crate::cryptography::error::CryptographyError;

use super::data::bets::UserContest;

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

    #[error("409: Cannot bet on both sides of a contest")]
    CannotBetOnBothSides,

    #[error("Message passed from snip-20 was not a contest bet")]
    MessageNotContestBet,

    #[error("User: {} has not bet on Contest: {}", .user_contest.get_address_as_str(), .user_contest.get_contest_id())]
    NoBetForUserContest { user_contest: UserContest },

    #[error("Contest with id: {0} Not Found")]
    ContestNotFound(u32),

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    ContestCryptagraphyError(#[from] CryptographyError),

    #[error("Time of close for contest with id: {0} has passed")]
    TimeOfClosePassed(u32),

    #[error("Time of resolve for contest with id: {contest_id} has yet to pass. Time of resolve: {time_of_resolve}, Current time: {current_time}")]
    TimeOfResolveHasYetToPassed {
        contest_id: u32,
        time_of_resolve: u64,
        current_time: u64,
    },

    #[error("The bet has already been paid")]
    BetAlreadyPaid,

    #[error("403: Cannot claim on a lost contest")]
    CannotClaimOnLostContest,

    #[error("Outcome with ID: {outcome_id}, was not found on Contest with ID: {contest_id}")]
    OutcomeNotFound {
        contest_id: u32,
        outcome_id: u8,
    },

    #[error("Outcome has already been set, and connot be reset")]
    CannotResetOutcome,

    #[error("412: Precondition Failed. Invalid Outcome ID found in contest with ID: {contest_id}")]
    InvalidOutcomeId {
        contest_id: u32,
    },
}

impl From<ContestError> for cosmwasm_std::StdError {
    fn from(contest_error: ContestError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contest Error: {}", contest_error.to_string()))
    }
}
