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

    #[error("Bet made without a sender")]
    CannotBetOnBothSides,

    #[error("Message passed from snip-20 was not a contest bet")]
    MessageNotContestBet,

    #[error("User: {} has not bet on Contest: {}", .user_contest.get_address_as_str(), .user_contest.get_contest_id())]
    NoBetForUserContest { user_contest: UserContest },

    #[error("Contest with id: {} Not Found", 0)]
    ContestNotFound(u32),

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    ContestCryptagraphyError(#[from] CryptographyError),

    #[error("Time of close for contest with id: {0} has passed")]
    TimeOfClosePassed(u32),

    #[error("Time of resolve for contest with id: {0} has passed")]
    TimeOfResolvePassed(u32),
}

impl From<ContestError> for cosmwasm_std::StdError {
    fn from(contest_error: ContestError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contest Error: {}", contest_error.to_string()))
    }
}
