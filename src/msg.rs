use cosmwasm_std::{Addr, Binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contest::data::{bets::UserContest, contest_info::ContestInfo};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub satoshis_palace: Addr,
    pub oracle_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateContest {
        contest_info: ContestInfo,
        contest_info_signature_hex: String,
        outcome_id: u8,
        sender: Option<Addr>,
        amount: Option<Uint128>,
    },
    BetContest {
        contest_id: u32,
        outcome_id: u8,
        sender: Option<Addr>,
        amount: Option<Uint128>,
    },
    Claim {
        contest_id: u32,
    },
    // SNIP-20 MSGs
    Register {
        reg_addr: Addr,
        reg_hash: String,
    },
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint128,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        msg: Binary,
    },
    Redeem {
        addr: String,
        hash: String,
        to: Addr,
        amount: Uint128,
        denom: Option<String>,
    },
    // Viewing Keys
    CreateViewingKey {
        entropy: String,
        padding: Option<String>,
    },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContest {
        contest_id: u32,
    },
    GetUserBet {
        user_contest: UserContest,
        key: String,
    },
    GetSnip20s {},
    //Contract specific snip-20s
    GetContestCreationMsgBinary {
        contest_info: ContestInfo,
        contest_info_signature_hex: String,
        outcome_id: u8,
    },
    GetBetContestMsgBinary {
        contest_id: u32,
        outcome_id: u8,
    }, //
}
