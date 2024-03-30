#[cfg(not(feature = "testing"))]
pub mod oracle {
    use cosmwasm_std::{QuerierWrapper, StdResult, Storage};
    use sp_secret_toolkit::oracle::{response::GetContestResultResponse, Oracle};

    pub const NULL_AND_VOID_CONTEST_RESULT: u8 = 0;

    pub fn query_contest_result(
        querier: &QuerierWrapper,
        storage: &dyn Storage,
        contest_id: &String,
    ) -> StdResult<GetContestResultResponse> {
        let oracle = Oracle::singleton_load(storage)?;
        oracle.get_contest_result(querier, contest_id)
    }
}

#[cfg(feature = "testing")]
pub mod oracle {
    pub const NULL_AND_VOID_CONTEST_RESULT: u8 = 0;

    use std::sync::atomic::{AtomicUsize, Ordering};

    static QUERY_CONTEST_RESULT_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    use std::sync::Mutex;

    use cosmwasm_std::{QuerierWrapper, StdResult, Storage};
    use sp_secret_toolkit::oracle::response::GetContestResultResponse;

    lazy_static! {
        // Use a Mutex to safely mutate the value across threads if necessary
        static ref MOCK_RESULT: Mutex<u8> = Mutex::new(1); // Default result
    }

    pub fn query_contest_result(
        _querier: &QuerierWrapper,
        _storage: &dyn Storage,
        _contest_id: &String,
    ) -> StdResult<GetContestResultResponse> {
        QUERY_CONTEST_RESULT_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        let result = *MOCK_RESULT.lock().unwrap(); // Retrieve the current mock result

        // Return a mock response with the result
        Ok(GetContestResultResponse { result })
    }

    pub fn set_mock_result(result: u8) {
        let mut mock_result = MOCK_RESULT.lock().unwrap();
        *mock_result = result;
    }

    pub fn reset_mock_result() {
        let mut mock_result = MOCK_RESULT.lock().unwrap();
        *mock_result = 1; // Reset to default value
    }

    pub fn reset_query_contest_result_call_count() {
        QUERY_CONTEST_RESULT_CALL_COUNT.store(0, Ordering::SeqCst);
    }

    pub fn assert_query_contest_result_call_count(expected: usize) {
        let calls = QUERY_CONTEST_RESULT_CALL_COUNT.load(Ordering::SeqCst);
        assert_eq!(calls, expected, "query_contest_result call count mismatch");
    }
}
