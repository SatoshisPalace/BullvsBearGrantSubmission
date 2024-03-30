use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UserInfoError {
    #[error("User info not found for address: {0}")]
    UserInfoNotFound(String),
}
impl From<UserInfoError> for cosmwasm_std::StdError {
    fn from(error: UserInfoError) -> Self {
        cosmwasm_std::StdError::generic_err(format!("UserInfoError Error: {}", error.to_string()))
    }
}
