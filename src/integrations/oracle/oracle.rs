use cosmwasm_std::{StdResult, Storage, QuerierWrapper};
use crate::integrations::contract::contract:: query_contract;
use super::{response::{GetContestStatusResponse, GetContestResultResponse}, query::QueryMsg, state::{config_read, OracleState}};

#[cfg(not(feature = "testing"))]
const BLOCK_SIZE: usize = 256;

// Query function for GetContestResult
#[cfg(not(feature = "testing"))]
pub fn query_contest_result(
    querier: &QuerierWrapper,
    storage: &dyn Storage,
    contest_id: u64,
) -> StdResult<GetContestResultResponse> {
    let state: OracleState = config_read(storage).load()?;
    let contract_info = &state.oracle_contract_info;

    let query_msg = QueryMsg::GetContestResult { contest_id };
    
    // query_contract(querier, contract_info, &query_msg)
    query_msg.query(querier, BLOCK_SIZE, contract_info.code_hash.clone(), contract_info.address.to_string())
}

// Query function for GetContestStatus
pub fn query_contest_status(
    querier: &QuerierWrapper,
    storage: &dyn Storage,
    contest_id: u64,
) -> StdResult<GetContestStatusResponse> {
    let state: OracleState = config_read(storage).load()?;
    let contract_info = &state.oracle_contract_info;

    let query_msg = QueryMsg::GetContestStatus { contest_id };

    query_contract(querier, contract_info, &query_msg)
}


#[cfg(feature = "testing")]
use std::sync::atomic::{Ordering, AtomicUsize};


#[cfg(feature = "testing")]
static QUERY_CONTEST_RESULT_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);



#[cfg(feature = "testing")]
use std::sync::Mutex;

#[cfg(feature = "testing")]
lazy_static! {
    // Use a Mutex to safely mutate the value across threads if necessary
    static ref MOCK_RESULT: Mutex<u8> = Mutex::new(1); // Default result
}

#[cfg(feature = "testing")]
pub fn query_contest_result(
    _querier: &QuerierWrapper,
    _storage: &dyn Storage,
    _contest_id: u64,) -> StdResult<GetContestResultResponse> {

    QUERY_CONTEST_RESULT_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    let result = *MOCK_RESULT.lock().unwrap(); // Retrieve the current mock result

    // Return a mock response with the result
    Ok(GetContestResultResponse { result })
}

#[cfg(feature = "testing")]
pub fn set_mock_result(result: u8) {
    let mut mock_result = MOCK_RESULT.lock().unwrap();
    *mock_result = result;
}

#[cfg(feature = "testing")]
pub fn reset_mock_result() {
    let mut mock_result = MOCK_RESULT.lock().unwrap();
    *mock_result = 1; // Reset to default value
}

#[cfg(feature = "testing")]
pub fn reset_query_contest_result_call_count() {
    QUERY_CONTEST_RESULT_CALL_COUNT.store(0, Ordering::SeqCst);
}

#[cfg(feature = "testing")]
pub fn assert_query_contest_result_call_count(expected: usize) {
    let calls = QUERY_CONTEST_RESULT_CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(calls, expected, "query_contest_result call count mismatch");
}