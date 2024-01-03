use cosmwasm_std::{StdResult, from_binary, WasmQuery, ContractInfo, to_binary, QueryRequest, QuerierWrapper};
use serde::{Serialize, de::DeserializeOwned};

// Modified query_contract to take querier as an argument
pub fn query_contract<Q: Serialize, R: DeserializeOwned>(
    querier: &QuerierWrapper,
    contract_info: &ContractInfo,
    query_msg: &Q,
) -> StdResult<R> {
    let wasm_query = WasmQuery::Smart {
        contract_addr: contract_info.address.to_string(),
        msg: to_binary(query_msg)?,
        code_hash: contract_info.code_hash.to_owned(),
    };
    let query_request = QueryRequest::Wasm(wasm_query);

    let binary_response = querier.query(&query_request)?;

    let response: R = from_binary(&binary_response)?;
    Ok(response)
}