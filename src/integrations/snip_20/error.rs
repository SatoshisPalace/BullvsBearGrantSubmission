use thiserror::Error;

use crate::msg::ExecuteMsg;

#[derive(Error, Debug, PartialEq)]
pub enum Snip20Error {
    #[error("Snip20 with address: {0}, is not known to this contract")]
    UnknownSnip20(String),

    #[error("Unsupported Method")]
    UnsupportedMethod(ExecuteMsg),

    #[error("There is no snip 20 at this index: {0}")]
    InvalidIndex(usize),

    #[error(transparent)]
    StandardError(#[from] cosmwasm_std::StdError),
}

impl From<Snip20Error> for cosmwasm_std::StdError {
    fn from(contest_error: Snip20Error) -> Self {
        cosmwasm_std::StdError::generic_err(format!("Snip-20 Error: {}", contest_error.to_string()))
    }
}
