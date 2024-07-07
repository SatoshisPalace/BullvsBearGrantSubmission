#[cfg(test)]
mod tests {
    use std::vec;

    use sp_secret_toolkit::price_feed::response::response_types::prices_by_ids::PricesByIdsResponse;

    use crate::{
        data::{contest_info::ContestId, state::FeePercent},
        msgs::query::commands::get_times_to_resolve::GetTimesToResolve,
        responses::query::response_types::times_to_resolve::TimesToResolveResponse,
        services::integrations::price_feed_service::pricefeed::set_oracle_result,
        tests::{
            constants::{
                AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
                AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_RESOLVE, BASE_FEE_PERCENT_DENOMINATOR,
                BASE_FEE_PERCENT_NUMERATOR,
            },
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn get_times_to_resolve_single() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        let expected_response = TimesToResolveResponse {
            times: vec![1571797500, 1571797800],
        };
        let ids = vec![ContestId::new("BTC".to_string(), 1571797500)];
        let command = GetTimesToResolve { contest_ids: ids };
        test_env.query_times_to_resolve(command, expected_response);
    }

    #[test]
    fn get_times_to_resolve_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        let expected_response = TimesToResolveResponse {
            times: vec![
                1571797500, 1571797800, 1571798100, 1571798400, 1571798700, 1571799000,
            ],
        };
        let ids = vec![
            ContestId::new("BTC".to_string(), 1571797500),
            ContestId::new("BTC".to_string(), 1571797800),
            ContestId::new("BTC".to_string(), 1571798100),
            ContestId::new("BTC".to_string(), 1571798400),
            ContestId::new("BTC".to_string(), 1571798700),
        ];
        let command = GetTimesToResolve { contest_ids: ids };
        test_env.query_times_to_resolve(command, expected_response);
    }
}
