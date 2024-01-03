use core::fmt;

use cosmwasm_std::{QueryRequest, WasmQuery, StdResult, QuerierWrapper, CustomQuery, StdError, to_binary};
use schemars::JsonSchema;
use secret_toolkit::utils::space_pad;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetDelegations returns the amount of staked NFTs this oracle has
    GetDelegations {
        name: String,
    },
    // GetOracleStatus returns if an oracle is in the active pool of oracles or not
    GetOracleStatus {
        name: String,
    },    
    // GetContestResult returns contest result status
    GetContestResult {
        contest_id: u64,
    },
    // GetContestStatus returns contest votes status
    GetContestStatus {
        contest_id: u64,
    }
}

impl QueryMsg {
    /// Returns a StdResult<T>, where T is the "Response" type that wraps the query answer
    ///
    /// # Arguments
    ///
    /// * `querier` - a reference to the Querier dependency of the querying contract
    /// * `block_size` - pad the message to blocks of this size
    /// * `callback_code_hash` - String holding the code hash of the contract being queried
    /// * `contract_addr` - address of the contract being queried
    pub fn query<C: CustomQuery, T: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper<C>,
        mut block_size: usize,
        code_hash: String,
        contract_addr: String,
    ) -> StdResult<T> {
        // can not have block size of 0
        if block_size == 0 {
            block_size = 1;
        }
        let mut msg = to_binary(self)?;
        space_pad(&mut msg.0, block_size);
        querier
            .query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr,
                code_hash,
                msg,
            }))
            .map_err(|err| {
                StdError::generic_err(format!("Error performing {} query: {}",self, err))
            })
    }
}

impl fmt::Display for QueryMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryMsg::GetDelegations { name: _ } => write!(f, "GetDelegations"),
            QueryMsg::GetOracleStatus { name: _ } => write!(f, "GetOracleStatus"),
            QueryMsg::GetContestResult { contest_id: _ } => write!(f, "GetContestResult"),
            QueryMsg::GetContestStatus { contest_id: _ } => write!(f, "GetContestStatus"),
        }
    }
}