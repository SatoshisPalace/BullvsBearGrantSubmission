#[cfg(not(feature = "testing"))]
pub mod pricefeed {
    use cosmwasm_std::{QuerierWrapper, StdResult, Storage};
    use sp_secret_toolkit::price_feed::{response::response_types::prices_by_ids::PricesByIdsResponse, PriceFeed};

    pub const NULL_AND_VOID_CONTEST_RESULT: u8 = 0;

    pub fn query_prices(
        querier: &QuerierWrapper,
        storage: &dyn Storage,
        prices: &Vec<u64>,
    ) -> StdResult<PricesByIdsResponse> {
        let price_feed = PriceFeed::singleton_load(storage)?;
        price_feed.get_prices_by_ids(querier, prices)
    }
}

#[cfg(feature = "testing")]
pub mod pricefeed {
    pub const NULL_AND_VOID_CONTEST_RESULT: u8 = 0;

    use std::str::FromStr;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    static QUERY_CONTEST_RESULT_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    use std::sync::Mutex;

    use cosmwasm_std::{Decimal, QuerierWrapper, StdError, StdResult, Storage};
    use sp_secret_toolkit::price_feed::data::price_posting::PricePosting;
    use sp_secret_toolkit::price_feed::response::response_types::prices_by_ids::PricesByIdsResponse;

    lazy_static! {
        // Use a Mutex to safely mutate the value across threads if necessary
        static ref MOCK_RESULT: Mutex<PricesByIdsResponse> = Mutex::new(PricesByIdsResponse{
            prices: vec![
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797500), 
                PricePosting::new(Decimal::from_str("58205.46").unwrap(), 1571797800)
                ]
        }); // Default result
    }

    // Atomic boolean to control error return
    static RETURN_ERROR: AtomicBool = AtomicBool::new(false);

    pub enum MockConfig {
        ReturnError(bool),
        MockResult(PricesByIdsResponse),
    }

    pub fn configure_mock(config: MockConfig) {
        match config {
            MockConfig::ReturnError(value) => {
                RETURN_ERROR.store(value, Ordering::SeqCst);
            }
            MockConfig::MockResult(result) => {
                let mut mock_result = MOCK_RESULT.lock().unwrap();
                *mock_result = result;
            }
        }
    }

    pub fn query_prices(
        _querier: &QuerierWrapper,
        _storage: &dyn Storage,
        _prices: &Vec<u64>,
    ) -> StdResult<PricesByIdsResponse> {
        //Will let us throw an error if wanted
        if RETURN_ERROR.load(Ordering::SeqCst) {
            return Err(StdError::generic_err(
                "Test error: query_contest_result failed",
            ));
        }

        QUERY_CONTEST_RESULT_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        let result = MOCK_RESULT.lock().unwrap().clone(); // Retrieve the current mock result

        // Return a mock response with the result
        Ok(result)
    }

    pub fn set_oracle_result(result: Option<PricesByIdsResponse>) {
        if result == None {
            RETURN_ERROR.store(false, Ordering::SeqCst);
        } else {
            let mut mock_result = MOCK_RESULT.lock().unwrap();
            *mock_result = result.unwrap();
        }
    }

    pub fn reset_mock_result() {
        let mut mock_result = MOCK_RESULT.lock().unwrap();
        *mock_result = PricesByIdsResponse {
            prices: vec![
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797500),
                PricePosting::new(Decimal::from_str("58205.46").unwrap(), 1571797800),
            ],
        }; // Reset to default value
    }

    pub fn reset_query_contest_result_call_count() {
        QUERY_CONTEST_RESULT_CALL_COUNT.store(0, Ordering::SeqCst);
    }

    pub fn assert_query_contest_result_call_count(expected: usize) {
        let calls = QUERY_CONTEST_RESULT_CALL_COUNT.load(Ordering::SeqCst);
        assert_eq!(calls, expected, "query_contest_result call count mismatch");
    }
}
