use cosmwasm_std::{Binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Snip20Msg {
    RegisterReceive {
        code_hash: String,
        padding: Option<String>,
    },
    Redeem {
        amount: Uint128,
        denom: String,
        padding: Option<String>,
    },
    Send {
        recipient: String,
        recipient_code_hash: Option<String>,
        amount: Uint128,
        msg: Option<Binary>,
    },
}

impl Snip20Msg {
    pub fn register_receive(code_hash: String) -> Self {
        Snip20Msg::RegisterReceive {
            code_hash,
            padding: None, // TODO add padding calculation
        }
    }

    pub fn redeem(amount: Uint128, denom: String) -> Self {
        Snip20Msg::Redeem {
            amount,
            denom,
            padding: None, // TODO add padding calculation
        }
    }
    pub fn send(
        recipient: String,
        recipient_code_hash: Option<String>,
        amount: Uint128,
        msg: Option<Binary>,
    ) -> Self {
        Snip20Msg::Send {
            recipient,
            recipient_code_hash,
            amount,
            msg: msg,
        }
    }
}
