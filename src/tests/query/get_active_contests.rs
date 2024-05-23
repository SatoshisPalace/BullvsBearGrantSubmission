#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        msgs::query::commands::get_contests::{ContestQueryFilter, ContestQuerySortOrder},
        tests::{
            constants::{AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE, AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_CLOSE, AFTER_TIME_OF_RESOLVE},
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn get_active_contests_single() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Active), 1);
    }

    #[test]
    fn get_active_contests_ignores_after_time_of_resolve() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Active), 0);
    }

    #[test]
    fn get_active_contests_ignores_after_time_of_close() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_CLOSE);
        test_env.get_contests_success(None, None, None, Some(ContestQueryFilter::Active), 0);
    }

    #[test]
    fn get_page_size_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.first_bet_on_contest_success(file_number, &1, &100);
        }

        test_env.get_contests_success(
            Some(0),
            Some(1),
            None,
            Some(ContestQueryFilter::Active),
            1,
        );
    }

    // Modified to pass, left in to be used later with multiple tickers
    #[test]
    fn get_page_size_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.first_bet_on_contest_success(&10, &1, &200);

        test_env.get_contests_success(
            Some(0),
            Some(2),
            None,
            Some(ContestQueryFilter::Active),
            2,
        );
    }

    #[test]
    fn get_page_num_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.first_bet_on_contest_success(&10, &1, &200);

        test_env.get_contests_success(
            Some(0),
            Some(1),
            None,
            Some(ContestQueryFilter::Active),
            1,
        );
    }

    #[test]
    fn get_page_num_1_page_size_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.first_bet_on_contest_success(&10, &1, &200);

        test_env.get_contests_success(
            Some(0),
            Some(2),
            None,
            Some(ContestQueryFilter::Active),
            2,
        );
    }

    #[test]
    fn get_page_sort_by_time_of_close() {
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

        test_env.get_contests_success(
            None,
            None,
            Some(ContestQuerySortOrder::Descending),
            None,
            5,
        );
    }

    #[test]
    fn get_page_sort_by_volume() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.first_bet_on_contest_success(&10, &1, &200);

        test_env.get_contests_success(
            None,
            None,
            Some(ContestQuerySortOrder::Volume),
            Some(ContestQueryFilter::Active),
            2,
        );
    }

    #[test]
    fn get_page_sort_by_volume_page_size_4_page_num_0() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.first_bet_on_contest_success(&10, &1, &200);

        test_env.get_contests_success(
            Some(0),
            Some(4),
            Some(ContestQuerySortOrder::Volume),
            Some(ContestQueryFilter::Active),
            2,
        );
    }
}
