#[cfg(test)]
mod tests {
    use crate::tests::{
        constants::{
            AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
            AFTER_TIME_OF_4_CLOSE,
        },
        test_env::tests::TestEnv,
    };
    use crate::{
        data::state::FeePercent,
        tests::constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
    };

    ////////TESTS////////
    #[test]
    fn get_contests_single() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.get_contests_by_ids_success(&vec![contest_file], Some(&1));
    }

    #[test]
    fn get_contests_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.first_bet_on_contest_success(&5, &1, &100);

        test_env.get_contests_by_ids_success(&contest_files, Some(&5)); // Expecting 5 contests across the files, if each file is supposed to hold one contest.
    }

    #[test]
    fn get_contests_missing_are_ignored() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let mut contest_files = vec![1, 2, 3, 4]; // Example vector of contest file numbers.

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.first_bet_on_contest_success(&4, &1, &100);

        contest_files.insert(4, 5);
        test_env.get_contests_by_ids_success(&contest_files, Some(&4)); // Expecting 5 contests across the files, if each file is supposed to hold one contest.
    }
}
