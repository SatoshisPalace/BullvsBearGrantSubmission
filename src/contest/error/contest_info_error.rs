use cosmwasm_std::Uint128;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContestInfoError {
    #[error("Contest Does Not Exist. Display Text: Failure to place bet. Cannot place bet on contest that does not exist.")]
    ContestDNE,

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
    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),
}

impl From<ContestInfoError> for cosmwasm_std::StdError {
    fn from(contest_error: ContestInfoError) -> Self {
        cosmwasm_std::StdError::generic_err(format!(
            "Contest Info Error: {}",
            contest_error.to_string()
        ))
    }
}
