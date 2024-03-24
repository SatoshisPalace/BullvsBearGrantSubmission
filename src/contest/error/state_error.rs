use thiserror::Error;

use crate::contest::data::bets::UserContest;

use super::contest_bet_summary_error::ContestBetSummaryError;

#[derive(Error, Debug, PartialEq)]
pub enum StateError {
    #[error("The bet has already been paid. Display Text: Failure to claim. Wallet has already claimed winnings on this contest.")]
    BetAlreadyPaid,
    #[error("User: {} has not bet on Contest: {}. Display Text: Failure to claim. Wallet has not placed a bet on this contest.", .user_contest.get_address_as_str(), .user_contest.get_contest_id())]
    NoBetForUserContest { user_contest: UserContest },

    #[error("403: Cannot claim on a lost contest. Display Text: Failure to claim. Wallet did not place a bet on the winning side.")]
    CannotClaimOnLostContest,

    #[error(transparent)]
    ContestBetSummaryError(#[from] ContestBetSummaryError),

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),
}

impl From<StateError> for cosmwasm_std::StdError {
    fn from(error: StateError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("State Error: {}", error.to_string()))
    }
}
