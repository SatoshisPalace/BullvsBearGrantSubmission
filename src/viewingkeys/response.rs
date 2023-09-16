use cosmwasm_std::{from_binary, to_binary, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ViewingKeyResponse {
    CreateViewingKey { key: String },
    SetViewingKey { status: ResponseStatus },
}

impl TryFrom<Binary> for ViewingKeyResponse {
    type Error = cosmwasm_std::StdError;

    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        from_binary(&binary)
    }
}
impl TryInto<Binary> for ViewingKeyResponse {
    type Error = cosmwasm_std::StdError;

    fn try_into(self) -> Result<Binary, Self::Error> {
        to_binary(&self)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub enum ResponseStatus {
    Success,
    Failure,
}
