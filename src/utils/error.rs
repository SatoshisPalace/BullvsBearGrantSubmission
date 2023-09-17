use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UtilError {
    #[error("Method Caller is not Contract")]
    ContractOnlyCall,
}

impl From<UtilError> for cosmwasm_std::StdError {
    fn from(util_error: UtilError) -> Self {
        return cosmwasm_std::StdError::GenericErr {
            msg: util_error.to_string(),
        };
    }
}
