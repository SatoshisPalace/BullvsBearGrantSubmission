use cosmwasm_std::Uint128;
use thiserror::Error;

use crate::cryptography::error::CryptographyError;

use super::data::bets::UserContest;

#[derive(Error, Debug, PartialEq)]
pub enum ContestError {
    #[error("Contest Does Not Exist. Display Text: Failure to place bet. Cannot place bet on contest that does not exist.")]
    ContestDNE,

    #[error("A contest with id: {0} already exists and cannot be reinitialized. Display Text: A contest with id: {0} already exists and cannot be reinitialized.")]
    ContestAlreadyExist(u32),

    #[error("Outcome Does Not Exist. Display Text: Failure to place bet. Cannot place bet on a side that does not exist.")]
    OutcomeDNE,

    #[error("Bet amount below minimum: attempted {attempted}, minimum required {minimum}")]
    BetBelowMinimum {
        attempted: Uint128,
        minimum: Uint128,
    },

    #[error("Bet made without amount. Display Text: Failure to place bet. Please enter a bet.")]
    BetHasNoAmount,

    #[error(
        "Bet made without a sender. Display Text: Failure to place bet. Bet Made without sender."
    )]
    BetHasNoSender,

    #[error("409: Cannot bet on both sides of a contest. Display Text: Failure to place bet. Current wallet can only be tied to one team.")]
    CannotBetOnBothSides,

    // #[error("Message passed from snip-20 was not a contest bet. Display Text:")]
    // MessageNotContestBet,
    #[error("User: {} has not bet on Contest: {}. Display Text: Failure to claim. Wallet has not placed a bet on this contest.", .user_contest.get_address_as_str(), .user_contest.get_contest_id())]
    NoBetForUserContest { user_contest: UserContest },

    #[error(
        "Contest with id: {0} Not Found. Display Text: Failure to claim. Contest does not exist."
    )]
    ContestNotFound(u32),

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    ContestCryptagraphyError(#[from] CryptographyError),

    #[error("Time of close for contest with id: {0} has passed. Display Text: Failure to place bet. Contest has closed.")]
    TimeOfClosePassed(u32),

    #[error("Time of resolve for contest with id: {contest_id} has yet to pass. Time of resolve: {time_of_resolve}, Current time: {current_time}. Display Text: Failure to claim. Contest has not been resolved.")]
    TimeOfResolveHasYetToPassed {
        contest_id: u32,
        time_of_resolve: u64,
        current_time: u64,
    },

    #[error("The bet has already been paid. Display Text: Failure to claim. Wallet has already claimed winnings on this contest.")]
    BetAlreadyPaid,

    #[error("403: Cannot claim on a lost contest. Display Text: Failure to claim. Wallet did not place a bet on the winning side.")]
    CannotClaimOnLostContest,

    #[error("Outcome with ID: {outcome_id}, was not found on Contest with ID: {contest_id}.")]
    OutcomeNotFound { contest_id: u32, outcome_id: u8 },

    #[error("Outcome has already been set, and connot be reset")]
    CannotResetOutcome,

    #[error("412: Precondition Failed. Invalid Outcome ID found in contest with ID: {contest_id}")]
    InvalidOutcomeId { contest_id: u32 },
}

impl From<ContestError> for cosmwasm_std::StdError {
    fn from(contest_error: ContestError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Contest Error: {}", contest_error.to_string()))
    }
}
