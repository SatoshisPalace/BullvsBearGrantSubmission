#[cfg(test)]
mod tests {
    use std::vec;

    use sp_secret_toolkit::price_feed::response::response_types::prices_by_ids::PricesByIdsResponse;

    use crate::{
        data::state::FeePercent,
        msgs::query::commands::get_contests::{ContestQueryFilter, ContestQuerySortOrder},
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
    fn get_unresolved_contests_single() {
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

        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    #[test]
    fn get_unresolved_contests_many() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 3);
    }

    // Test for filtering unresolved contests where the contest is still open
    #[test]
    fn test_filter_unresolved_contests_contest_still_open() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is still open
    #[test]
    fn test_filter_unresolved_contests_contest_still_open_multiple() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        // Create multiple open contests
        let contest_files = vec![1, 2, 3];
        for contest_file in &contest_files {
            test_env.first_bet_on_contest_success(contest_file, &1, &100);
        }

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is resolved
    #[test]
    fn test_filter_unresolved_contests_contest_resolved() {
        let mut test_env = TestEnv::new();

        // Do not create any contests
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is resolved
    #[test]
    fn test_filter_unresolved_contests_contest_resolved_multiple() {
        let mut test_env = TestEnv::new();

        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_filter_unresolved_contests_contest_unresolved() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;

        // Create a closed contest awaiting results
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    // Test for filtering unresolved contests where the contest is past expiration window
    #[test]
    fn test_filter_unresolved_contests_contest_past_expiration_window() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_filter_unresolved_contests_contest_unresolved_multiple() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 3);
    }

    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_sort_contests_by_volume() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.first_bet_on_contest_success(&2, &1, &200);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.first_bet_on_contest_success(&3, &1, &300);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.first_bet_on_contest_success(&4, &1, &400);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.first_bet_on_contest_success(&5, &1, &500);

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(
            None,
            None,
            Some(ContestQuerySortOrder::Volume),
            Some(ContestQueryFilter::Unresolved),
            3,
        );
    }

    #[test]
    fn get_page_size_1() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        test_env.get_contests_success(
            Some(0),
            Some(1),
            None,
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    #[test]
    fn get_page_size_2() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        test_env.get_contests_success(
            Some(0),
            Some(2),
            None,
            Some(ContestQueryFilter::Unresolved),
            2,
        );
    }

    #[test]
    fn get_page_num_1() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        test_env.get_contests_success(
            Some(0),
            Some(1),
            None,
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    #[test]
    fn get_page_num_1_page_size_2() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        test_env.get_contests_success(
            Some(0),
            Some(2),
            None,
            Some(ContestQueryFilter::Unresolved),
            2,
        );
    }

    #[test]
    fn get_unresolved_contests_many_other_user() {
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

        let oracle_result = PricesByIdsResponse { prices: vec![] };
        set_oracle_result(Some(oracle_result));

        test_env.set_sender("user2".to_owned());

        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 3);
    }
}
