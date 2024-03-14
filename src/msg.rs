use cosmwasm_std::{Addr, Binary, ContractInfo, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contest::data::{bets::UserContest, contest_info::ContestInfo};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub satoshis_palace: Addr,
    pub oracle_contract_info: ContractInfo,
    pub snip20: ContractInfo,
    pub entropy: Binary,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Claim {
        contest_id: u32,
    },
    // SNIP-20 MSGs
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint128,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        msg: Binary,
    },
    // Viewing Keys
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
    SetMinBet {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvokeMsg {
    CreateContest {
        contest_info: ContestInfo,
        contest_info_signature_hex: String,
        outcome_id: u8,
        user: Addr,
        amount: Option<Uint128>,
    },
    BetContest {
        contest_id: u32,
        outcome_id: u8,
        user: Addr,
        amount: Option<Uint128>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContest {
        contest_id: u32,
    },
    GetContests {
        contest_ids: Vec<u32>,
    },
    GetUserBet {
        user_contest: UserContest,
        key: String,
    },
    GetContestResult {
        contest_id: u32,
    },
    GetMinBet {},

    GetSnip20 {},
}
