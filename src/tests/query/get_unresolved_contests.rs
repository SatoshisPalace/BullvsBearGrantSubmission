#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        msgs::query::commands::get_contests::{ContestQueryFilter, ContestQuerySortOrder},
        services::integrations::price_feed_service::pricefeed::{configure_mock, MockConfig},
        tests::{
            constants::{
                AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
                AFTER_TIME_OF_4_CLOSE,
            },
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn get_unresolved_contests_single() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        configure_mock(MockConfig::ReturnError(true));
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    #[test]
    fn get_unresolved_contests_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

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

        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    // Test for filtering unresolved contests where the contest is still open
    #[test]
    fn test_filter_unresolved_contests_contest_still_open() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        configure_mock(MockConfig::ReturnError(true));
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is still open
    #[test]
    fn test_filter_unresolved_contests_contest_still_open_multiple() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // Create multiple open contests
        let contest_files = vec![1, 2, 3];
        for contest_file in &contest_files {
            test_env.first_bet_on_contest_success(contest_file, &1, &100);
        }

        // Set oracle to return an error to simulate unresolved contest
        configure_mock(MockConfig::ReturnError(true));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is resolved
    #[test]
    fn test_filter_unresolved_contests_contest_resolved() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // Do not create any contests

        // Set oracle to return an error to simulate unresolved contest
        configure_mock(MockConfig::ReturnError(true));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is resolved
    #[test]
    fn test_filter_unresolved_contests_contest_resolved_multiple() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // Expecting length 0 so create no contests

        // Set oracle to return an error to simulate unresolved contest
        configure_mock(MockConfig::ReturnError(true));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 0);
    }

    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_filter_unresolved_contests_contest_unresolved() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;

        // Create a closed contest awaiting results
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        // Set oracle to not return an error to simulate resolved contest
        configure_mock(MockConfig::ReturnError(true));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    // Modified to work, left in to be used later with multiple tickers
    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_filter_unresolved_contests_contest_unresolved_multiple() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // Create multiple open contests
        let contest_files = vec![1, 2, 3];
        for contest_file in &contest_files {
            test_env.first_bet_on_contest_success(contest_file, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        // Set oracle to not return an error to simulate resolved contest
        configure_mock(MockConfig::ReturnError(true));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    // Modified to work, left in to be used later with multiple tickers
    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_filter_unresolved_contests_several_contest_types_multiple() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // Create multiple open contests
        let contest_files = vec![1, 2, 3];
        for contest_file in &contest_files {
            test_env.first_bet_on_contest_success(contest_file, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        // Set oracle to not return an error to simulate resolved contest
        configure_mock(MockConfig::ReturnError(false));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }

    // Modified to work, left in to be used later with multiple tickers
    // Test for filtering unresolved contests where the contest is unresolved
    #[test]
    fn test_sort_contests_by_volume() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // Create multiple open contests
        let contest_files = vec![1, 2, 3];
        for contest_file in &contest_files {
            test_env.first_bet_on_contest_success(
                contest_file,
                &1,
                &((*contest_file as u128) * 100),
            );
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        // Set oracle to not return an error to simulate resolved contest
        configure_mock(MockConfig::ReturnError(false));

        // Call get_contests_success with filter for unresolved contests
        test_env.get_contests_success(
            None,
            None,
            Some(ContestQuerySortOrder::Volume),
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    // Modified to work, left in to be used later with multiple tickers
    #[test]
    fn get_page_size_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.get_contests_success(
            Some(0),
            Some(1),
            None,
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    // Modified to work, left in to be used later with multiple tickers
    #[test]
    fn get_page_size_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.get_contests_success(
            Some(0),
            Some(2),
            None,
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    // Modified to work, left in to be used later with multiple tickers
    #[test]
    fn get_page_num_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.get_contests_success(
            Some(0),
            Some(1),
            None,
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    // Modified to work, left in to be used later with multiple tickers
    #[test]
    fn get_page_num_1_page_size_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.get_contests_success(
            Some(0),
            Some(2),
            None,
            Some(ContestQueryFilter::Unresolved),
            1,
        );
    }

    #[test]
    fn get_active_contests_many_other_user() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.first_bet_on_contest_success(&5, &1, &100);

        test_env.set_sender("user2".to_owned());

        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Unresolved), 1);
    }
}
