use thiserror::Error;

use crate::data::bets::UserContest;

use super::contest_bet_summary_error::ContestBetSummaryError;

#[derive(Error, Debug, PartialEq)]
pub enum BetError {
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

    #[error("409: Cannot bet on both sides of a contest. Display Text: Failure to place bet. Current wallet can only be tied to one team.")]
    CannotBetOnBothSides,
}

impl From<BetError> for cosmwasm_std::StdError {
    fn from(error: BetError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Bet Error: {}", error.to_string()))
    }
}
