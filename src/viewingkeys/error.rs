use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ViewingKeyError {
    #[error("This Query Message does not require a viewing key.")]
    InvalidQueryMessage,

    #[error(transparent)]
    InvalidAddress(#[from] cosmwasm_std::StdError),
}

impl From<ViewingKeyError> for cosmwasm_std::StdError {
    fn from(viewing_key_error: ViewingKeyError) -> Self {
        return cosmwasm_std::StdError::GenericErr {
            msg: viewing_key_error.to_string(),
        };
    }
}
