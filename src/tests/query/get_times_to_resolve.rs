#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        data::state::FeePercent,
        responses::query::response_types::times_to_resolve::TimesToResolveResponse,
        services::integrations::price_feed_service::pricefeed::{configure_mock, MockConfig},
        tests::{
            constants::{
                AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
                AFTER_TIME_OF_4_CLOSE, BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR,
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
        configure_mock(MockConfig::ReturnError(true));

        let expected_response = TimesToResolveResponse {
            times: vec![1571797500, 1571797800],
        };
        test_env.query_times_to_resolve(expected_response);
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

        configure_mock(MockConfig::ReturnError(true));
        let expected_response = TimesToResolveResponse {
            times: vec![1571797500, 1571797800, 1571798100, 1571798400],
        };
        test_env.query_times_to_resolve(expected_response);
    }
}
