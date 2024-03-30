use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContestInfoError {
    #[error("Bet made without amount. Display Text: Failure to place bet. Please enter a bet.")]
    BetHasNoAmount,

    #[error(
        "Bet made without a sender. Display Text: Failure to place bet. Bet Made without sender."
    )]
    BetHasNoSender,

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
